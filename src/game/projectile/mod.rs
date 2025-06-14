pub mod fireball;
pub mod laser;
pub mod missile;

use crate::game::GameLayer;
use crate::game::combat::faction::Faction;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<ProjectileConfig>,
        Projectile,
        Thruster,
        Homing,
        RotateWithVelocity,
        Growth,
    )>();

    app.add_plugins((fireball::plugin, laser::plugin, missile::plugin));
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct ProjectileConfig {
    missile_damage: f32,
    missile_initial_position_spread: Vec2,
    missile_initial_angle_spread: f32,
    missile_initial_speed: f32,
    missile_initial_speed_spread: f32,
    missile_max_speed: f32,
    missile_thruster_force: Vec2,
    missile_homing_approach: f32,
    missile_homing_target_spread: Vec2,
    missile_growth_rate: Vec2,
    missile_initial_scale: Vec2,
    missile_max_scale: Vec2,
    missile_oscillate_amplitude: Vec2,
    missile_oscillate_phase: Vec2,
    missile_oscillate_rate: Vec2,

    laser_damage: f32,
    laser_initial_position_spread: Vec2,
    laser_initial_angle_spread: f32,
    laser_initial_speed: f32,
    laser_initial_speed_spread: f32,
    laser_max_speed: f32,
    laser_thruster_force: Vec2,
    laser_homing_approach: f32,
    laser_homing_target_spread: Vec2,

    fireball_damage: f32,
    fireball_initial_position: Vec2,
    fireball_initial_position_spread: Vec2,
    fireball_initial_angle_spread: f32,
    fireball_initial_speed: f32,
    fireball_initial_speed_spread: f32,
    fireball_max_speed: f32,
    fireball_thruster_force: Vec2,
    fireball_homing_approach: f32,
    fireball_homing_target_spread: Vec2,
    fireball_growth_rate: Vec2,
    fireball_initial_scale: Vec2,
    fireball_max_scale: Vec2,
}

impl Config for ProjectileConfig {
    const FILE: &'static str = "projectile.ron";
}

impl Default for ProjectileConfig {
    fn default() -> Self {
        Self {
            missile_damage: 0.0,
            missile_initial_position_spread: Vec2::ZERO,
            missile_initial_speed: 0.0,
            missile_initial_speed_spread: 0.0,
            missile_initial_angle_spread: 0.0,
            missile_max_speed: f32::INFINITY,
            missile_thruster_force: Vec2::ZERO,
            missile_homing_approach: 1.0,
            missile_homing_target_spread: Vec2::ZERO,
            missile_growth_rate: Vec2::ZERO,
            missile_initial_scale: Vec2::ONE,
            missile_max_scale: Vec2::ONE,
            missile_oscillate_amplitude: Vec2::ZERO,
            missile_oscillate_phase: Vec2::ZERO,
            missile_oscillate_rate: Vec2::ZERO,

            laser_damage: 0.0,
            laser_initial_position_spread: Vec2::ZERO,
            laser_initial_speed: 0.0,
            laser_initial_speed_spread: 0.0,
            laser_initial_angle_spread: 0.0,
            laser_max_speed: f32::INFINITY,
            laser_thruster_force: Vec2::ZERO,
            laser_homing_approach: 1.0,
            laser_homing_target_spread: Vec2::ZERO,

            fireball_damage: 0.0,
            fireball_initial_position: Vec2::ZERO,
            fireball_initial_position_spread: Vec2::ZERO,
            fireball_initial_speed: 0.0,
            fireball_initial_speed_spread: 0.0,
            fireball_initial_angle_spread: 0.0,
            fireball_max_speed: f32::INFINITY,
            fireball_thruster_force: Vec2::ZERO,
            fireball_homing_approach: 1.0,
            fireball_homing_target_spread: Vec2::ZERO,
            fireball_growth_rate: Vec2::ZERO,
            fireball_initial_scale: Vec2::ONE,
            fireball_max_scale: Vec2::ONE,
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Projectile;

impl Configure for Projectile {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

fn projectile(faction: Faction, transform: Transform) -> impl Bundle {
    (
        Projectile,
        faction,
        RigidBody::Dynamic,
        Mass(1.0),
        ExternalForce::ZERO.with_persistence(false),
        CollisionLayers::new(GameLayer::Default, faction.opponent().layer()),
        CollisionEventsEnabled,
        transform,
        GlobalTransform::from(transform),
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Thruster {
    force: Vec2,
}

impl Configure for Thruster {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_thruster
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn apply_thruster(mut thruster_query: Query<(&mut ExternalForce, &GlobalTransform, &Thruster)>) {
    for (mut force, gt, thruster) in &mut thruster_query {
        force.apply_force(gt.rotation().to_rot2() * thruster.force);
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Homing {
    target: Entity,
    offset: Vec2,
    approach: f32,
}

impl Configure for Homing {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_homing
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn apply_homing(
    time: Res<Time>,
    mut homing_query: Query<(
        &mut LinearVelocity,
        Option<&MaxLinearSpeed>,
        &GlobalTransform,
        &Homing,
    )>,
    target_query: Query<&GlobalTransform>,
) {
    for (mut velocity, maybe_max_speed, gt, homing) in &mut homing_query {
        cq!(velocity.0 != Vec2::ZERO);
        let target_gt = cq!(target_query.get(homing.target));

        // Calculate the required rotation to point velocity towards target position.
        let target_pos = target_gt.translation().xy() + homing.offset;
        let delta = target_pos - gt.translation().xy();
        let full_rotation = velocity.angle_to(delta);

        // Approach the rotation exponentially.
        let time_scale = if let Some(max_speed) = maybe_max_speed {
            if max_speed.0 == 0.0 {
                1.0
            } else {
                velocity.length() / max_speed.0
            }
        } else {
            1.0
        };
        let decay = homing.approach.powf(time.delta_secs() * time_scale);
        let rotation = full_rotation * (1.0 - decay).clamp(0.0, 1.0);
        velocity.0 = Vec2::from_angle(rotation).rotate(velocity.0);
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct RotateWithVelocity;

impl Configure for RotateWithVelocity {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            rotate_with_velocity
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn rotate_with_velocity(
    mut projectile_query: Query<(&mut Transform, &LinearVelocity), With<RotateWithVelocity>>,
) {
    for (mut transform, velocity) in &mut projectile_query {
        cq!(velocity.0 != Vec2::ZERO);
        transform.rotation = Quat::radians(velocity.to_angle());
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Growth {
    rate: Vec2,
    max_scale: Vec2,
}

impl Configure for Growth {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_growth
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn apply_growth(time: Res<Time>, mut growth_query: Query<(&mut Transform, &Growth)>) {
    for (mut transform, growth) in &mut growth_query {
        let delta = growth.rate * time.delta_secs();
        transform.scale = (transform.scale.xy() + delta)
            .min(growth.max_scale)
            .extend(transform.scale.z);
    }
}
