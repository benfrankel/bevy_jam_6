use crate::animation::oscillate::Oscillate;
use crate::animation::shake::Shake;
use crate::animation::shake::Trauma;
use crate::combat::damage::OnDamage;
use crate::combat::death::OnDeath;
use crate::combat::faction::Faction;
use crate::combat::health::Health;
use crate::combat::health::HealthBar;
use crate::combat::health::health_bar;
use crate::core::camera::CameraRoot;
use crate::core::physics::GameLayer;
use crate::deck::PlayerDeck;
use crate::hud::Hud;
use crate::hud::HudConfig;
use crate::hud::helm::hand::HandIndex;
use crate::level::Level;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;
use crate::util::math::ScalingTrauma;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<ShipConfig>,
        PlayerShip,
        PlayerShipBody,
        EnemyShip,
        Weapon,
    )>();

    app.add_systems(
        Update,
        (
            tilt_player_ship_with_velocity.in_set(UpdateSystems::Update),
            navigate_player_ship_toward_selected_module
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        ),
    );
}

pub fn player_ship(
    ship_config: &ShipConfig,
    game_assets: &GameplayAssets,
    health: f32,
) -> impl Bundle {
    let weapons = ship_config.player_weapons.clone();
    let image = game_assets.player_ship.clone();

    (
        Name::new("PlayerShip"),
        PlayerShip,
        Faction::Player,
        Health::new(health),
        Visibility::default(),
        RigidBody::Kinematic,
        MaxLinearSpeed(ship_config.player_speed_max),
        ship_config.player_shake,
        Oscillate::new(
            ship_config.player_oscillate_amplitude,
            ship_config.player_oscillate_phase,
            ship_config.player_oscillate_rate,
        ),
        children![
            (
                health_bar(),
                Transform::from_translation(ship_config.player_health_bar_offset.extend(0.1))
                    .with_scale(ship_config.player_health_bar_size.extend(1.0)),
            ),
            (
                Name::new("Body"),
                PlayerShipBody,
                Transform::default(),
                Sprite::from_image(image),
                Collider::rectangle(80.0, 10.0),
                CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
                Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                    let rotation = Rot2::turn_fraction(0.25).to_quat();
                    for pos in weapons {
                        parent.spawn((
                            weapon(),
                            Transform::from_translation(pos.extend(-0.1)).with_rotation(rotation),
                        ));
                    }
                })),
            ),
        ],
        Patch(|entity| {
            entity.observe(shake_player_ship_on_damage);
            entity.observe(shake_screen_on_damage);
        }),
    )
}

fn shake_player_ship_on_damage(
    trigger: Trigger<OnDamage>,
    mut trauma_query: Query<&mut Trauma>,
    ship_config: ConfigRef<ShipConfig>,
) {
    let ship_config = r!(ship_config.get());
    let target = r!(trigger.get_target());
    let mut trauma = r!(trauma_query.get_mut(target));
    trauma.0 += ship_config.player_damage_trauma.sample(trigger.0);
}

pub fn enemy_ship(
    ship_config: &ShipConfig,
    game_assets: &GameplayAssets,
    health: f32,
) -> impl Bundle {
    let weapons = ship_config.enemy_weapons.clone();
    let health_bar_transform =
        Transform::from_translation(ship_config.enemy_health_bar_offset.extend(0.1))
            .with_scale(ship_config.enemy_health_bar_size.extend(1.0));

    (
        Name::new("EnemyShip"),
        EnemyShip,
        Faction::Enemy,
        Health::new(health),
        Sprite::from_image(game_assets.enemy_ship.clone()),
        RigidBody::Dynamic,
        Dominance(1),
        Mass(1.0),
        Collider::rectangle(167.0, 15.0),
        CollisionLayers::new(GameLayer::Enemy, LayerMask::ALL),
        ship_config.enemy_shake,
        Oscillate::new(
            ship_config.enemy_oscillate_amplitude,
            ship_config.enemy_oscillate_phase,
            ship_config.enemy_oscillate_rate,
        ),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent.spawn((health_bar(), health_bar_transform));

            let rotation = Rot2::turn_fraction(0.75).to_quat();
            for pos in weapons {
                parent.spawn((
                    weapon(),
                    Transform::from_translation(pos.extend(-0.1)).with_rotation(rotation),
                ));
            }
        })),
        Patch(|entity| {
            entity.observe(survive_on_one_health);
            entity.observe(shake_enemy_ship_on_damage);
        }),
    )
}

fn survive_on_one_health(
    trigger: Trigger<OnDeath>,
    level: CurrentRef<Level>,
    mut health_query: Query<&mut Health>,
) {
    let target = r!(trigger.get_target());
    let mut health = r!(health_query.get_mut(target));
    rq!(!level.is_in(&Level(9)));
    health.current = 1.0;
}

fn shake_enemy_ship_on_damage(
    trigger: Trigger<OnDamage>,
    mut trauma_query: Query<&mut Trauma>,
    ship_config: ConfigRef<ShipConfig>,
) {
    let ship_config = r!(ship_config.get());
    let target = r!(trigger.get_target());
    let mut trauma = r!(trauma_query.get_mut(target));
    trauma.0 += ship_config.enemy_damage_trauma.sample(trigger.0);
}

fn weapon() -> impl Bundle {
    (
        Name::new("Weapon"),
        Weapon,
        #[cfg(feature = "dev")]
        Collider::triangle(vec2(0.0, -2.0), vec2(0.0, 2.0), vec2(8.0, 0.0)),
    )
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct ShipConfig {
    player_weapons: Vec<Vec2>,
    player_health_bar_offset: Vec2,
    player_health_bar_size: Vec2,
    player_accel_max: f32,
    player_speed_approach: f32,
    player_speed_sensitivity: f32,
    player_speed_max: f32,
    player_tilt_sensitivity: f32,
    player_tilt_max: f32,
    player_oscillate_amplitude: Vec2,
    player_oscillate_phase: Vec2,
    player_oscillate_rate: Vec2,
    player_shake: Shake,
    player_damage_trauma: ScalingTrauma,

    enemy_weapons: Vec<Vec2>,
    enemy_health_bar_offset: Vec2,
    enemy_health_bar_size: Vec2,
    enemy_oscillate_amplitude: Vec2,
    enemy_oscillate_phase: Vec2,
    enemy_oscillate_rate: Vec2,
    enemy_shake: Shake,
    enemy_damage_trauma: ScalingTrauma,
}

impl Config for ShipConfig {
    const FILE: &'static str = "ship.ron";
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerShip;

impl Configure for PlayerShip {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerShipBody;

impl Configure for PlayerShipBody {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct EnemyShip;

impl Configure for EnemyShip {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Weapon;

impl Configure for Weapon {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

fn navigate_player_ship_toward_selected_module(
    ship_config: ConfigRef<ShipConfig>,
    time: Res<Time>,
    camera_root: Res<CameraRoot>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    hand_index_query: Query<(&HandIndex, &GlobalTransform, &ComputedNode)>,
    player_ship: Single<(&mut LinearVelocity, &GlobalTransform), With<PlayerShip>>,
    player_deck: Res<PlayerDeck>,
) {
    let ship_config = r!(ship_config.get());
    let (camera, camera_gt) = r!(camera_query.get(camera_root.primary));
    let target_pos = if let Some((_, target_gt, target_computed_node)) = hand_index_query
        .iter()
        .find(|(index, ..)| index.0 == player_deck.hand_idx)
    {
        let viewport_pos = camera
            .physical_viewport_rect()
            .map(|x| x.min)
            .unwrap_or_default()
            .as_vec2();
        let target_pos_viewport = (viewport_pos + target_gt.translation().xy())
            * target_computed_node.inverse_scale_factor;
        r!(camera.viewport_to_world_2d(camera_gt, target_pos_viewport))
    } else {
        vec2(61.0, -46.0)
    };

    let (mut velocity, gt) = player_ship.into_inner();
    let delta = target_pos.x - gt.translation().x;
    let target_speed = ship_config.player_speed_sensitivity * delta;
    let delta_speed = target_speed - velocity.x;
    let dt = time.delta_secs();
    let decay = ship_config.player_speed_approach.powf(dt);
    let approach = (1.0 - decay).clamp(0.0, 1.0) * delta_speed;
    let accel = approach.clamp(
        -ship_config.player_accel_max * dt,
        ship_config.player_accel_max * dt,
    );

    velocity.x += accel;
}

fn tilt_player_ship_with_velocity(
    ship_config: ConfigRef<ShipConfig>,
    player_ship: Single<(&Children, &LinearVelocity, &MaxLinearSpeed), With<PlayerShip>>,
    mut transform_query: Query<&mut Transform, Without<HealthBar>>,
) {
    let ship_config = r!(ship_config.get());
    let (children, velocity, max_speed) = player_ship.into_inner();
    let angle = (-ship_config.player_tilt_sensitivity * velocity.x / max_speed.0)
        .clamp(-ship_config.player_tilt_max, ship_config.player_tilt_max);
    let rotation = Quat::degrees(angle);

    for &child in children {
        let mut transform = cq!(transform_query.get_mut(child));
        transform.rotation = rotation;
    }
}

fn shake_screen_on_damage(
    trigger: Trigger<OnDamage>,
    hud_config: ConfigRef<HudConfig>,
    camera_root: Res<CameraRoot>,
    hud_query: Query<Entity, With<Hud>>,
    mut trauma_query: Query<&mut Trauma>,
) {
    let hud_config = r!(hud_config.get());
    let mut camera_trauma = r!(trauma_query.get_mut(camera_root.primary));
    camera_trauma.0 += hud_config.camera_player_damage_trauma.sample(trigger.0);

    for entity in &hud_query {
        let mut trauma = cq!(trauma_query.get_mut(entity));
        trauma.0 += hud_config.hud_player_damage_trauma.sample(trigger.0);
    }
}
