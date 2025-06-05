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
    // Calculate initial position.
    transform.translation += (missile_config.initial_position_spread
        * vec2(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)))
    .extend(0.0);

    // Calculate initial rotation.
    let angle = transform.rotation.to_rot2().as_degrees()
        + missile_config.initial_angle_spread * rng.gen_range(-1.0..=1.0);
    let angle = angle.to_radians();
    transform.rotation = Quat::from_rotation_z(angle);

    // Calculate initial velocity.
    let speed = missile_config.initial_speed
        + missile_config.initial_speed_spread * rng.gen_range(-1.0..=1.0);
    let velocity = speed.max(1.0) * Vec2::from_angle(angle);

    (
        Name::new("Missile"),
        IsMissile,
        Sprite::from_image(missile_assets.image.clone()),
        Damage(missile_config.damage * flux),
        faction,
        RigidBody::Dynamic,
        LinearVelocity(velocity),
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
    initial_position_spread: Vec2,
    initial_angle_spread: f32,
    initial_speed: f32,
    initial_speed_spread: f32,
    max_speed: f32,
    thruster_force: f32,
    homing_approach: f32,
    homing_target_spread: Vec2,
}

impl Config for MissileConfig {
    const FILE: &'static str = "missile.ron";
}

impl Default for MissileConfig {
    fn default() -> Self {
        Self {
            damage: 0.0,
            initial_position_spread: Vec2::ZERO,
            initial_speed: 0.0,
            initial_speed_spread: 0.0,
            initial_angle_spread: 0.0,
            max_speed: f32::INFINITY,
            thruster_force: 0.0,
            homing_approach: 1.0,
            homing_target_spread: Vec2::ZERO,
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
                apply_missile_homing.in_set(UpdateSystems::Update),
                rotate_with_velocity.in_set(UpdateSystems::Update),
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
    let rng = &mut thread_rng();
    for (mut velocity, gt, faction) in &mut missile_query {
        cq!(velocity.0 != Vec2::ZERO);

        // Calculate the required rotation to point velocity towards target position.
        let target_gt = match faction {
            Faction::Player => *enemy_ship,
            Faction::Enemy => *player_ship,
        };
        let target_pos = target_gt.translation().xy()
            + missile_config.homing_target_spread
                * vec2(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
        let delta = target_pos - gt.translation().xy();
        let full_rotation = velocity.angle_to(delta);

        // Approach the rotation exponentially.
        let time_scale = velocity.length() / missile_config.max_speed;
        let decay = missile_config
            .homing_approach
            .powf(time.delta_secs() * time_scale);
        let rotation = full_rotation * (1.0 - decay).clamp(0.0, 1.0);
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
