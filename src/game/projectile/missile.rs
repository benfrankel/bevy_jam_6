use core::f32;

use crate::game::GameLayer;
use crate::game::combat::damage::Damage;
use crate::game::combat::faction::Faction;
use crate::game::projectile::ProjectileAssets;
use crate::game::projectile::ProjectileConfig;
use crate::game::ship::IsEnemyShip;
use crate::game::ship::IsPlayerShip;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsMissile>();
}

pub fn missile(
    mut rng: impl Rng,
    projectile_config: &ProjectileConfig,
    projectile_assets: &ProjectileAssets,
    faction: Faction,
    flux: f32,
    mut transform: Transform,
) -> impl Bundle {
    // Calculate initial position.
    transform.translation += (projectile_config.missile_initial_position_spread
        * vec2(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)))
    .extend(0.0);

    // Calculate initial rotation.
    let angle = transform.rotation.to_rot2().as_degrees()
        + projectile_config.missile_initial_angle_spread * rng.gen_range(-1.0..=1.0);
    let angle = angle.to_radians();
    transform.rotation = Quat::from_rotation_z(angle);

    // Calculate initial velocity.
    let speed = projectile_config.missile_initial_speed
        + projectile_config.missile_initial_speed_spread * rng.gen_range(-1.0..=1.0);
    let velocity = speed.max(1.0) * Vec2::from_angle(angle);

    (
        Name::new("Missile"),
        IsMissile,
        Sprite::from_image(projectile_assets.missile_image.clone()),
        Damage(projectile_config.missile_damage * flux),
        faction,
        RigidBody::Dynamic,
        LinearVelocity(velocity),
        MaxLinearSpeed(projectile_config.missile_max_speed),
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
            )
                .in_set(PausableSystems),
        );
    }
}

fn apply_missile_thrusters(
    projectile_config: ConfigRef<ProjectileConfig>,
    mut missile_query: Query<(&mut ExternalForce, &GlobalTransform), With<IsMissile>>,
) {
    let projectile_config = r!(projectile_config.get());
    for (mut force, gt) in &mut missile_query {
        force.apply_force(projectile_config.missile_thruster_force * gt.rotation().to_dir2());
    }
}

fn apply_missile_homing(
    time: Res<Time>,
    projectile_config: ConfigRef<ProjectileConfig>,
    mut missile_query: Query<(&mut LinearVelocity, &GlobalTransform, &Faction), With<IsMissile>>,
    player_ship: Single<&GlobalTransform, With<IsPlayerShip>>,
    enemy_ship: Single<&GlobalTransform, With<IsEnemyShip>>,
) {
    let projectile_config = r!(projectile_config.get());
    let rng = &mut thread_rng();
    for (mut velocity, gt, faction) in &mut missile_query {
        cq!(velocity.0 != Vec2::ZERO);

        // Calculate the required rotation to point velocity towards target position.
        let target_gt = match faction {
            Faction::Player => *enemy_ship,
            Faction::Enemy => *player_ship,
        };
        let target_pos = target_gt.translation().xy()
            + projectile_config.missile_homing_target_spread
                * vec2(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
        let delta = target_pos - gt.translation().xy();
        let full_rotation = velocity.angle_to(delta);

        // Approach the rotation exponentially.
        let time_scale = velocity.length() / projectile_config.missile_max_speed;
        let decay = projectile_config
            .missile_homing_approach
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
