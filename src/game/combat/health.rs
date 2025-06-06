use avian2d::math::Vector;

use crate::game::combat::death::OnDeath;
use crate::game::ship::IsPlayerShip;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<HealthConfig>,
        Health,
        IsHealthBar,
        OnHeal,
        HealthAssets,
        IsHealObject,
    )>();
}

pub fn health_bar() -> impl Bundle {
    (Name::new("HealthBar"), IsHealthBar)
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct HealthConfig {
    health_bar_color_ramp: Vec<Color>,
}

impl Config for HealthConfig {
    const FILE: &'static str = "health.ron";
}

impl HealthConfig {
    fn health_bar_color(&self, t: f32) -> Color {
        let n = self.health_bar_color_ramp.len();
        let t = t * (n - 1) as f32;
        let lo = t as usize;
        let hi = lo + 1;
        let t = t.fract();

        if hi >= n {
            self.health_bar_color_ramp[n - 1]
        } else {
            self.health_bar_color_ramp[lo].mix(&self.health_bar_color_ramp[hi], t)
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }

    pub fn heal(&mut self, amount: f32) {
        self.current += amount;
        // TODO: Play an animation
        // TODO: Play an animation
    }
}

impl Configure for Health {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, detect_death.in_set(UpdateSystems::Update));
    }
}

#[derive(Event, Reflect, Debug)]
pub struct OnHeal(pub f32);

impl Configure for OnHeal {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(increase_health_on_action);
        app.add_observer(display_healed_feedback);
    }
}

fn detect_death(mut commands: Commands, health_query: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, health) in &health_query {
        rq!(health.current <= f32::EPSILON);
        commands.entity(entity).trigger(OnDeath);
    }
}

/// Reads from the [`Health`] component on its parent entity.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Sprite)]
pub struct IsHealthBar;

impl Configure for IsHealthBar {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, sync_health_bar.in_set(UpdateSystems::SyncLate));
    }
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct HealthAssets {
    #[asset(path = "image/health/heal.png")]
    heal_image: Handle<Image>,
}

impl Configure for HealthAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsHealObject;

impl Configure for IsHealObject {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_fade_out_heal);
    }
}

fn sync_health_bar(
    health_config: ConfigRef<HealthConfig>,
    health_query: Query<&Health>,
    mut health_bar_query: Query<(&ChildOf, &mut Sprite), With<IsHealthBar>>,
) {
    let health_config = r!(health_config.get());
    for (child_of, mut sprite) in &mut health_bar_query {
        let health = c!(health_query.get(child_of.parent()));
        let t = health.current.max(0.0) / health.max;

        sprite.custom_size = Some(vec2(t, 1.0));
        sprite.color = health_config.health_bar_color(t);
    }
}

fn increase_health_on_action(trigger: Trigger<OnHeal>, mut health_query: Query<&mut Health>) {
    let ship = r!(trigger.get_target());
    let mut health = r!(health_query.get_mut(ship));
    health.heal(trigger.0);
}

fn display_healed_feedback(
    _: Trigger<OnHeal>,
    mut commands: Commands,
    health_assets: Res<HealthAssets>,
    player_ship: Single<&Transform, With<IsPlayerShip>>,
) {
    commands.spawn((
        IsHealObject,
        Sprite::from_image(health_assets.heal_image.clone()),
        Transform::from_xyz(
            player_ship.translation.x,
            player_ship.translation.y + 30.,
            -1.,
        ),
        RigidBody::Dynamic,
        LinearVelocity(Vector::new(0., 10.)),
    ));
}

fn apply_fade_out_heal(
    mut commands: Commands,
    query: Query<(Entity, &mut Sprite), With<IsHealObject>>,
) {
    for (entity, mut sprite) in query {
        if sprite.color.alpha() < 0.01 {
            commands.entity(entity).despawn();
        }

        sprite.color = Color::srgba(1., 1., 1., sprite.color.alpha() * 0.92);
    }
}
