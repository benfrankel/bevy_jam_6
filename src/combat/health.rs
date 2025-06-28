use crate::combat::death::Dead;
use crate::combat::death::DieOnLifetime;
use crate::combat::death::FadeOutOnDeath;
use crate::combat::death::OnDeath;
use crate::level::Level;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<HealthConfig>,
        Health,
        HealthBarFill,
        HealthBarLabel,
        OnHeal,
        HealPopup,
    )>();
}

pub fn health_bar(offset: Vec2, size: Vec2) -> impl Bundle {
    (
        Name::new("HealthBar"),
        Transform::from_translation(offset.extend(0.1)),
        Visibility::default(),
        children![
            (
                Name::new("HealthBarLabel"),
                HealthBarLabel,
                Text2d::default(),
                TextFont {
                    font: FONT_HANDLE,
                    font_size: 8.0,
                    ..default()
                },
                ThemeColorForText(vec![ThemeColor::BodyText]),
                Transform::from_translation(vec3(0.0, 1.5, 0.1)),
            ),
            (
                Name::new("HealthBarFill"),
                HealthBarFill,
                Sprite::default(),
                Transform::from_scale(size.extend(1.0)),
            ),
        ],
    )
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
    heal_popup_fade_delay: f32,
    heal_popup_fade_duration: f32,
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
pub struct HealthBarFill;

impl Configure for HealthBarFill {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, sync_health_bar.in_set(UpdateSystems::SyncLate));
    }
}

fn sync_health_bar(
    health_config: ConfigRef<HealthConfig>,
    mut fill_query: Query<(&ChildOf, &mut Sprite), With<HealthBarFill>>,
    parent_query: Query<&ChildOf>,
    health_query: Query<&Health>,
) {
    let health_config = r!(health_config.get());
    for (child_of, mut sprite) in &mut fill_query {
        let grandchild_of = c!(parent_query.get(child_of.parent()));
        let health = c!(health_query.get(grandchild_of.parent()));

        let t = health.current.max(0.0) / health.max;
        sprite.custom_size = Some(vec2(t, 1.0));
        sprite.color = health_config.health_bar_color(t);
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct HealthBarLabel;

impl Configure for HealthBarLabel {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, sync_health_label.in_set(UpdateSystems::SyncLate));
    }
}

fn sync_health_label(
    health_query: Query<&Health>,
    parent_query: Query<&ChildOf>,
    mut label_query: Query<(&ChildOf, &mut Text2d), With<HealthBarLabel>>,
) {
    for (child_of, mut text) in &mut label_query {
        let grandchild_of = c!(parent_query.get(child_of.parent()));
        let health = c!(health_query.get(grandchild_of.parent()));
        text.0 = health.current.to_string();
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
    transform.scale *= vec3(scale, scale, 1.0);

    commands.spawn((
        HealPopup,
        Text2d::new("+".to_string()),
        TextFont {
            font: FONT_HANDLE,
            font_size: health_config.heal_popup_font_size,
            ..default()
        },
        TextColor::from(health_config.heal_popup_font_color),
        DieOnLifetime(health_config.heal_popup_fade_delay),
        FadeOutOnDeath {
            duration: health_config.heal_popup_fade_duration,
        },
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
