use core::f32;

use crate::game::GameAssets;
use crate::game::GameLayer;
use crate::game::combat::damage::Damage;
use crate::game::combat::faction::Faction;
use crate::game::projectile::ProjectileConfig;
use crate::game::ship::IsEnemyShip;
use crate::game::ship::IsPlayerShip;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsLaser>();
}

pub fn laser(
    mut rng: impl Rng,
    projectile_config: &ProjectileConfig,
    game_assets: &GameAssets,
    faction: Faction,
    flux: f32,
    mut transform: Transform,
) -> impl Bundle {
    // Calculate initial position.
    transform.translation += (projectile_config.laser_initial_position_spread
        * vec2(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)))
    .extend(0.0);

    // Calculate initial rotation.
    let angle = transform.rotation.to_rot2().as_degrees()
        + projectile_config.laser_initial_angle_spread * rng.gen_range(-1.0..=1.0);
    let angle = angle.to_radians();
    transform.rotation = Quat::from_rotation_z(angle);

    // Calculate initial velocity.
    let speed = projectile_config.laser_initial_speed
        + projectile_config.laser_initial_speed_spread * rng.gen_range(-1.0..=1.0);
    let velocity = speed.max(1.0) * Vec2::from_angle(angle);

    (
        Name::new("Laser"),
        IsLaser,
        Sprite::from_image(game_assets.laser.clone()),
        Damage(projectile_config.laser_damage * flux),
        faction,
        RigidBody::Dynamic,
        LinearVelocity(velocity),
        MaxLinearSpeed(projectile_config.laser_max_speed),
        ExternalForce::ZERO.with_persistence(false),
        Collider::capsule_endpoints(3.0, vec2(-3.5, 0.0), vec2(3.5, 0.0)),
        CollisionLayers::new(GameLayer::Default, faction.opponent().layer()),
        CollisionEventsEnabled,
        transform,
        GlobalTransform::from(transform),
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsLaser;

impl Configure for IsLaser {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            (
                apply_laser_thrusters.in_set(UpdateSystems::Update),
                apply_laser_homing.in_set(UpdateSystems::Update),
                rotate_with_velocity.in_set(UpdateSystems::Update),
            )
                .in_set(PausableSystems),
        );
    }
}

fn apply_laser_thrusters(
    projectile_config: ConfigRef<ProjectileConfig>,
    mut laser_query: Query<(&mut ExternalForce, &GlobalTransform), With<IsLaser>>,
) {
    let projectile_config = r!(projectile_config.get());
    for (mut force, gt) in &mut laser_query {
        force.apply_force(projectile_config.laser_thruster_force * gt.rotation().to_dir2());
    }
}

fn apply_laser_homing(
    time: Res<Time>,
    projectile_config: ConfigRef<ProjectileConfig>,
    mut laser_query: Query<(&mut LinearVelocity, &GlobalTransform, &Faction), With<IsLaser>>,
    player_ship: Single<&GlobalTransform, With<IsPlayerShip>>,
    enemy_ship: Single<&GlobalTransform, With<IsEnemyShip>>,
) {
    let projectile_config = r!(projectile_config.get());
    let rng = &mut thread_rng();
    for (mut velocity, gt, faction) in &mut laser_query {
        cq!(velocity.0 != Vec2::ZERO);

        // Calculate the required rotation to point velocity towards target position.
        let target_gt = match faction {
            Faction::Player => *enemy_ship,
            Faction::Enemy => *player_ship,
        };
        let target_pos = target_gt.translation().xy()
            + projectile_config.laser_homing_target_spread
                * vec2(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
        let delta = target_pos - gt.translation().xy();
        let full_rotation = velocity.angle_to(delta);

        // Approach the rotation exponentially.
        let time_scale = velocity.length() / projectile_config.laser_max_speed;
        let decay = projectile_config
            .laser_homing_approach
            .powf(time.delta_secs() * time_scale);
        let rotation = full_rotation * (1.0 - decay).clamp(0.0, 1.0);
        velocity.0 = Vec2::from_angle(rotation).rotate(velocity.0);
    }
}

fn rotate_with_velocity(mut laser_query: Query<(&mut Transform, &LinearVelocity), With<IsLaser>>) {
    for (mut transform, velocity) in &mut laser_query {
        cq!(velocity.0 != Vec2::ZERO);
        transform.rotation = Quat::from_rotation_z(velocity.to_angle());
    }
}
