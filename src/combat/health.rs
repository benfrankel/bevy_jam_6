use crate::combat::death::Dead;
use crate::combat::death::OnDeath;
use crate::level::Level;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<HealthConfig>,
        Health,
        HealthBar,
        OnHeal,
        HealPopup,
    )>();
}

pub fn health_bar() -> impl Bundle {
    (Name::new("HealthBar"), HealthBar)
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct HealthConfig {
    health_bar_color_ramp: Vec<Color>,
    heal_popup_font_size: f32,
    heal_popup_font_color: Color,
    heal_popup_offset: Vec2,
    heal_popup_offset_spread: Vec2,
    heal_popup_velocity: Vec2,
    heal_popup_fade_rate: f32,
    heal_popup_scale: f32,
    heal_popup_scale_factor: f32,
    heal_popup_scale_max: f32,
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
}

impl Configure for Health {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            (
                detect_death_from_health.in_set(UpdateSystems::Update),
                clamp_health.in_set(UpdateSystems::SyncLate),
            ),
        );
    }
}

fn detect_death_from_health(
    mut commands: Commands,
    health_query: Query<(Entity, &Health), (Changed<Health>, Without<Dead>)>,
) {
    for (entity, health) in &health_query {
        rq!(health.current <= f32::EPSILON);
        commands.entity(entity).trigger(OnDeath);
    }
}

fn clamp_health(mut health_query: Query<&mut Health, Changed<Health>>) {
    for mut health in &mut health_query {
        health.current = health.current.clamp(0.0, health.max);
    }
}

/// Reads from the [`Health`] component on its parent entity.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Sprite)]
pub struct HealthBar;

impl Configure for HealthBar {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, sync_health_bar.in_set(UpdateSystems::SyncLate));
    }
}

fn sync_health_bar(
    health_config: ConfigRef<HealthConfig>,
    health_query: Query<&Health>,
    mut health_bar_query: Query<(&ChildOf, &mut Sprite), With<HealthBar>>,
) {
    let health_config = r!(health_config.get());
    for (child_of, mut sprite) in &mut health_bar_query {
        let health = c!(health_query.get(child_of.parent()));
        let t = health.current.max(0.0) / health.max;

        sprite.custom_size = Some(vec2(t, 1.0));
        sprite.color = health_config.health_bar_color(t);
    }
}

#[derive(Event, Reflect, Debug)]
pub struct OnHeal(pub f32);

impl Configure for OnHeal {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(increase_health_on_heal);
    }
}

fn increase_health_on_heal(trigger: Trigger<OnHeal>, mut health_query: Query<&mut Health>) {
    let ship = r!(trigger.get_target());
    let mut health = r!(health_query.get_mut(ship));
    health.current += trigger.0;
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct HealPopup;

impl Configure for HealPopup {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(spawn_heal_popup_on_heal);
        app.add_systems(
            Update,
            apply_fade_out_to_heal_popup
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn spawn_heal_popup_on_heal(
    trigger: Trigger<OnHeal>,
    mut commands: Commands,
    health_config: ConfigRef<HealthConfig>,
    transform_query: Query<&Transform>,
) {
    let target = r!(trigger.get_target());
    let health_config = r!(health_config.get());

    // Randomize position.
    let rng = &mut thread_rng();
    let mut transform = *r!(transform_query.get(target));
    transform.translation += (health_config.heal_popup_offset
        + health_config.heal_popup_offset_spread * rng.gen_range(-1.0..=1.0))
    .extend(5.0);

    // Scale with number.
    let scale = (health_config.heal_popup_scale
        * health_config
            .heal_popup_scale_factor
            .max(1.0)
            .powf(trigger.0))
    .min(health_config.heal_popup_scale_max);
    transform.scale = (transform.scale.xy() * Vec2::splat(scale)).extend(transform.scale.z);

    commands.spawn((
        HealPopup,
        Text2d::new("+".to_string()),
        TextFont {
            font: FONT_HANDLE,
            font_size: health_config.heal_popup_font_size,
            ..default()
        },
        TextColor::from(health_config.heal_popup_font_color),
        transform,
        RigidBody::Kinematic,
        LinearVelocity(health_config.heal_popup_velocity),
        DespawnOnExitState::<Level>::default(),
        children![(
            Name::new("Number"),
            TextSpan(trigger.0.to_string()),
            TextFont {
                font: BOLD_FONT_HANDLE,
                font_size: health_config.heal_popup_font_size,
                ..default()
            },
            TextColor::from(health_config.heal_popup_font_color),
        )],
    ));
}

fn apply_fade_out_to_heal_popup(
    mut commands: Commands,
    time: Res<Time>,
    health_config: ConfigRef<HealthConfig>,
    heal_popup_query: Query<(Entity, &Children), With<HealPopup>>,
    mut text_color_query: Query<&mut TextColor>,
) {
    let health_config = r!(health_config.get());
    for (entity, children) in heal_popup_query {
        if let Ok(mut text_color) = text_color_query.get_mut(entity) {
            let alpha =
                text_color.0.alpha() - health_config.heal_popup_fade_rate * time.delta_secs();
            text_color.0 = text_color.0.with_alpha(alpha);
            if text_color.0.alpha() < f32::EPSILON {
                commands.entity(entity).try_despawn();
            }
        }

        for &child in children {
            let mut text_color = cq!(text_color_query.get_mut(child));
            let alpha =
                text_color.0.alpha() - health_config.heal_popup_fade_rate * time.delta_secs();
            text_color.0 = text_color.0.with_alpha(alpha);
        }
    }
}
