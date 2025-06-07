use crate::animation::oscillate::Oscillate;
use crate::animation::shake::NodeShake;
use crate::animation::shake::Shake;
use crate::animation::shake::ShakeWithScreen;
use crate::core::camera::CameraRoot;
use crate::game::GameAssets;
use crate::game::GameLayer;
use crate::game::combat::damage::OnDamage;
use crate::game::combat::death::OnDeath;
use crate::game::combat::faction::Faction;
use crate::game::combat::health::Health;
use crate::game::combat::health::IsHealthBar;
use crate::game::combat::health::health_bar;
use crate::game::deck::PlayerDeck;
use crate::game::hud::HudConfig;
use crate::game::hud::helm::HandIndex;
use crate::game::level::Level;
use crate::menu::Menu;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<ShipConfig>,
        IsPlayerShip,
        IsPlayerShipBody,
        IsEnemyShip,
        IsWeapon,
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

pub fn player_ship(ship_config: &ShipConfig, game_assets: &GameAssets, health: f32) -> impl Bundle {
    let weapons = ship_config.player_weapons.clone();
    let image = game_assets.player_ship.clone();

    (
        Name::new("PlayerShip"),
        IsPlayerShip,
        Faction::Player,
        Health::new(health),
        Visibility::default(),
        RigidBody::Kinematic,
        MaxLinearSpeed(ship_config.player_speed_max),
        Shake::default(),
        children![
            (
                health_bar(),
                Transform::from_translation(ship_config.player_health_bar_offset.extend(0.1))
                    .with_scale(ship_config.player_health_bar_size.extend(1.0)),
            ),
            (
                Name::new("Body"),
                IsPlayerShipBody,
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
            entity.observe(lose_level_on_death);
            entity.observe(shake_player_ship_on_damage);
            entity.observe(shake_screen_on_damage);
        }),
    )
}

fn lose_level_on_death(_: Trigger<OnDeath>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Defeat);
    menu.acquire();
}

fn shake_player_ship_on_damage(
    trigger: Trigger<OnDamage>,
    mut shake: Single<&mut Shake, With<IsPlayerShip>>,
    hud_config: ConfigRef<HudConfig>,
) {
    let hud_config = r!(hud_config.get());

    let factor = hud_config
        .player_ship_shake_damage_factor
        .powf(trigger.0.max(hud_config.player_ship_shake_damage_min) - 1.0);
    shake.amplitude = hud_config.player_ship_shake_amplitude;
    shake.trauma += hud_config.player_ship_shake_trauma * factor;
    shake.decay = hud_config.player_ship_shake_decay;
    shake.exponent = hud_config.player_ship_shake_exponent;
}

pub fn enemy_ship(ship_config: &ShipConfig, game_assets: &GameAssets, health: f32) -> impl Bundle {
    let weapons = ship_config.enemy_weapons.clone();
    let health_bar_transform =
        Transform::from_translation(ship_config.enemy_health_bar_offset.extend(0.1))
            .with_scale(ship_config.enemy_health_bar_size.extend(1.0));

    (
        Name::new("EnemyShip"),
        IsEnemyShip,
        Faction::Enemy,
        Health::new(health),
        Sprite::from_image(game_assets.enemy_ship.clone()),
        RigidBody::Kinematic,
        Collider::rectangle(167.0, 15.0),
        CollisionLayers::new(GameLayer::Enemy, LayerMask::ALL),
        Shake::default(),
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
            entity.observe(win_level_on_death);
            entity.observe(shake_enemy_ship_on_damage);
        }),
    )
}

fn win_level_on_death(
    _: Trigger<OnDeath>,
    mut menu: ResMut<NextStateStack<Menu>>,
    level: NextRef<Level>,
) {
    if r!(level.get()).0 == 9 {
        menu.push(Menu::Victory);
    } else {
        menu.push(Menu::LevelUp);
    }
    menu.acquire();
}

fn shake_enemy_ship_on_damage(
    trigger: Trigger<OnDamage>,
    mut shake: Single<&mut Shake, With<IsEnemyShip>>,
    hud_config: ConfigRef<HudConfig>,
) {
    let hud_config = r!(hud_config.get());

    let factor = hud_config
        .enemy_ship_shake_damage_factor
        .powf(trigger.0.max(hud_config.enemy_ship_shake_damage_min) - 1.0);
    shake.amplitude = hud_config.enemy_ship_shake_amplitude;
    shake.trauma += hud_config.enemy_ship_shake_trauma * factor;
    shake.decay = hud_config.enemy_ship_shake_decay;
    shake.exponent = hud_config.enemy_ship_shake_exponent;
}

fn weapon() -> impl Bundle {
    (
        Name::new("Weapon"),
        IsWeapon,
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

    enemy_weapons: Vec<Vec2>,
    enemy_health_bar_offset: Vec2,
    enemy_health_bar_size: Vec2,
    enemy_oscillate_amplitude: Vec2,
    enemy_oscillate_phase: Vec2,
    enemy_oscillate_rate: Vec2,
}

impl Config for ShipConfig {
    const FILE: &'static str = "ship.ron";
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsPlayerShip;

impl Configure for IsPlayerShip {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsPlayerShipBody;

impl Configure for IsPlayerShipBody {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsEnemyShip;

impl Configure for IsEnemyShip {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsWeapon;

impl Configure for IsWeapon {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

fn navigate_player_ship_toward_selected_module(
    time: Res<Time>,
    camera_root: Res<CameraRoot>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    hand_index_query: Query<(&HandIndex, &GlobalTransform, &ComputedNode)>,
    ship_config: ConfigRef<ShipConfig>,
    player_ship: Single<(&mut LinearVelocity, &GlobalTransform), With<IsPlayerShip>>,
    player_deck: Res<PlayerDeck>,
) {
    let ship_config = r!(ship_config.get());
    let (camera, camera_gt) = r!(camera_query.get(camera_root.primary));
    let (_, target_gt, target_computed_node) = rq!(hand_index_query
        .iter()
        .find(|(index, ..)| index.0 == player_deck.selected_idx));
    let viewport_pos = target_gt.translation().xy() * target_computed_node.inverse_scale_factor;
    let target_pos = r!(camera.viewport_to_world_2d(camera_gt, viewport_pos));

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
    player_ship: Single<(&Children, &LinearVelocity, &MaxLinearSpeed), With<IsPlayerShip>>,
    mut transform_query: Query<&mut Transform, Without<IsHealthBar>>,
) {
    let ship_config = r!(ship_config.get());
    let (children, velocity, max_speed) = player_ship.into_inner();
    let angle = (-ship_config.player_tilt_sensitivity * velocity.x / max_speed.0)
        .clamp(-ship_config.player_tilt_max, ship_config.player_tilt_max);
    let rotation = Quat::from_rotation_z(angle.to_radians());

    for &child in children {
        let mut transform = cq!(transform_query.get_mut(child));
        transform.rotation = rotation;
    }
}

fn shake_screen_on_damage(
    trigger: Trigger<OnDamage>,
    hud_config: ConfigRef<HudConfig>,
    mut shake_query: Query<&mut Shake, With<ShakeWithScreen>>,
    mut node_shake_query: Query<&mut NodeShake, With<ShakeWithScreen>>,
) {
    let hud_config = r!(hud_config.get());
    let factor = hud_config
        .screen_shake_damage_factor
        .powf(trigger.0.max(hud_config.screen_shake_damage_min) - 1.0);

    for mut shake in &mut shake_query {
        shake.amplitude = hud_config.camera_screen_shake_amplitude;
        shake.trauma += hud_config.camera_screen_shake_trauma * factor;
        shake.decay = hud_config.camera_screen_shake_decay;
        shake.exponent = hud_config.camera_screen_shake_exponent;
    }
    for mut node_shake in &mut node_shake_query {
        node_shake.amplitude = hud_config.ui_screen_shake_amplitude;
        node_shake.trauma += hud_config.ui_screen_shake_trauma * factor;
        node_shake.decay = hud_config.ui_screen_shake_decay;
        node_shake.exponent = hud_config.ui_screen_shake_exponent;
    }
}
