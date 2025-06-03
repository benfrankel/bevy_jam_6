use core::f32;

use crate::game::GameLayer;
use crate::game::combat::damage::Damage;
use crate::game::combat::faction::Faction;
use crate::game::ship::IsEnemyShip;
use crate::game::ship::IsPlayerShip;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<MissileConfig>, MissileAssets, IsMissile)>();
}

pub fn missile(
    mut rng: impl Rng,
    missile_config: &MissileConfig,
    missile_assets: &MissileAssets,
    faction: Faction,
    flux: f32,
    mut transform: Transform,
) -> impl Bundle {
    let mut angle = transform.rotation.to_rot2().as_degrees();
    angle += missile_config.angle_spread * rng.gen_range(-1.0..=1.0);
    transform.rotation = Rot2::degrees(angle).to_quat();

    let speed = missile_config.speed + missile_config.speed_spread * rng.gen_range(-1.0..=1.0);
    let initial_velocity = speed * Vec2::from_angle(angle.to_radians());

    (
        Name::new("Missile"),
        IsMissile,
        Sprite::from_image(missile_assets.image.clone()),
        Damage(missile_config.damage * flux.max(1.0)),
        faction,
        RigidBody::Dynamic,
        LinearVelocity(initial_velocity),
        MaxLinearSpeed(missile_config.max_speed),
        ExternalForce::ZERO.with_persistence(false),
        Collider::capsule_endpoints(3.0, vec2(-3.5, 0.0), vec2(3.5, 0.0)),
        CollisionLayers::new(GameLayer::Default, faction.opponent().layer()),
        CollisionEventsEnabled,
        transform,
        GlobalTransform::from(transform),
    )
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct MissileConfig {
    damage: f32,
    speed: f32,
    speed_spread: f32,
    max_speed: f32,
    angle_spread: f32,
    thruster_force: f32,
    homing_coefficient: f32,
}

impl Config for MissileConfig {
    const FILE: &'static str = "missile.ron";
}

impl Default for MissileConfig {
    fn default() -> Self {
        Self {
            damage: 0.0,
            speed: 0.0,
            speed_spread: 0.0,
            angle_spread: 0.0,
            max_speed: f32::INFINITY,
            thruster_force: 0.0,
            homing_coefficient: 0.0,
        }
    }
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct MissileAssets {
    #[asset(path = "image/projectile/missile.png")]
    image: Handle<Image>,
}

impl Configure for MissileAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsMissile;

impl Configure for IsMissile {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            (
                apply_missile_thrusters.in_set(UpdateSystems::Update),
                rotate_with_velocity.in_set(UpdateSystems::Update),
                apply_missile_homing.in_set(UpdateSystems::Update),
            ),
        );
    }
}

fn apply_missile_thrusters(
    missile_config: ConfigRef<MissileConfig>,
    mut missile_query: Query<(&mut ExternalForce, &GlobalTransform), With<IsMissile>>,
) {
    let missile_config = r!(missile_config.get());
    for (mut force, gt) in &mut missile_query {
        force.apply_force(missile_config.thruster_force * gt.rotation().to_dir2());
    }
}

fn apply_missile_homing(
    time: Res<Time>,
    missile_config: ConfigRef<MissileConfig>,
    mut missile_query: Query<(&mut LinearVelocity, &GlobalTransform, &Faction), With<IsMissile>>,
    player_ship: Single<&GlobalTransform, With<IsPlayerShip>>,
    enemy_ship: Single<&GlobalTransform, With<IsEnemyShip>>,
) {
    let missile_config = r!(missile_config.get());
    for (mut velocity, gt, faction) in &mut missile_query {
        cq!(velocity.0 != Vec2::ZERO);

        let target_gt = match faction {
            Faction::Player => *enemy_ship,
            Faction::Enemy => *player_ship,
        };
        let delta = target_gt.translation().xy() - gt.translation().xy();
        let offset = velocity.angle_to(delta);
        let decay = missile_config.homing_coefficient.powf(time.delta_secs());
        let rotation = offset * (1.0 - decay).clamp(0.0, 1.0);

        velocity.0 = Vec2::from_angle(rotation).rotate(velocity.0);
    }
}

fn rotate_with_velocity(
    mut missile_query: Query<(&mut Transform, &LinearVelocity), With<IsMissile>>,
) {
    for (mut transform, velocity) in &mut missile_query {
        cq!(velocity.0 != Vec2::ZERO);
        transform.rotation = Quat::from_rotation_z(velocity.to_angle());
    }
}
