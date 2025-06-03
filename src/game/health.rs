use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<HealthConfig>, Health, HealthBar)>();
}

pub fn health_bar(size: Vec2) -> impl Bundle {
    (Name::new("HealthBar"), HealthBar { size })
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct HealthConfig {
    color_ramp: Vec<Color>,
}

impl Config for HealthConfig {
    const FILE: &'static str = "health.ron";
}

impl HealthConfig {
    fn color(&self, t: f32) -> Color {
        let n = self.color_ramp.len();
        let t = t * (n - 1) as f32;
        let lo = t as usize;
        let hi = lo + 1;
        let t = t.fract();

        if hi >= n {
            self.color_ramp[n - 1]
        } else {
            self.color_ramp[lo].mix(&self.color_ramp[hi], t)
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
    }
}

/// Reads from the [`Health`] component on its parent entity.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Sprite)]
pub struct HealthBar {
    pub size: Vec2,
}

impl Configure for HealthBar {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, sync_health_bar.in_set(UpdateSystems::SyncLate));
    }
}

fn sync_health_bar(
    health_config: ConfigRef<HealthConfig>,
    health_query: Query<&Health>,
    mut health_bar_query: Query<(&HealthBar, &ChildOf, &mut Sprite)>,
) {
    let health_config = r!(health_config.get());
    for (health_bar, child_of, mut sprite) in &mut health_bar_query {
        let health = c!(health_query.get(child_of.parent()));
        let t = health.current / health.max;

        sprite.custom_size = Some(vec2(t * health_bar.size.x, health_bar.size.y));
        sprite.color = health_config.color(t);
    }
}
