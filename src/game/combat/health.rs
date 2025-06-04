use crate::game::combat::death::OnDeath;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<HealthConfig>, Health, IsHealthBar, OnHeal)>();
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
