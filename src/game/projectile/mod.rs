pub mod fireball;
pub mod laser;
pub mod missile;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<ProjectileConfig>, ProjectileAssets)>();

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
    missile_thruster_force: f32,
    missile_homing_approach: f32,
    missile_homing_target_spread: Vec2,
    missile_growth_rate: f32,
    missile_initial_scale: f32,
    missile_max_scale: f32,

    laser_damage: f32,
    laser_initial_position_spread: Vec2,
    laser_initial_angle_spread: f32,
    laser_initial_speed: f32,
    laser_initial_speed_spread: f32,
    laser_max_speed: f32,
    laser_thruster_force: f32,
    laser_homing_approach: f32,
    laser_homing_target_spread: Vec2,

    fireball_damage: f32,
    fireball_initial_position_spread: Vec2,
    fireball_initial_angle_spread: f32,
    fireball_initial_speed: f32,
    fireball_initial_speed_spread: f32,
    fireball_max_speed: f32,
    fireball_thruster_force: f32,
    fireball_homing_approach: f32,
    fireball_homing_target_spread: Vec2,
    fireball_growth_rate: f32,
    fireball_initial_scale: f32,
    fireball_max_scale: f32,
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
            missile_thruster_force: 0.0,
            missile_homing_approach: 1.0,
            missile_homing_target_spread: Vec2::ZERO,
            missile_growth_rate: 0.0,
            missile_initial_scale: 1.0,
            missile_max_scale: 1.0,

            laser_damage: 0.0,
            laser_initial_position_spread: Vec2::ZERO,
            laser_initial_speed: 0.0,
            laser_initial_speed_spread: 0.0,
            laser_initial_angle_spread: 0.0,
            laser_max_speed: f32::INFINITY,
            laser_thruster_force: 0.0,
            laser_homing_approach: 1.0,
            laser_homing_target_spread: Vec2::ZERO,

            fireball_damage: 0.0,
            fireball_initial_position_spread: Vec2::ZERO,
            fireball_initial_speed: 0.0,
            fireball_initial_speed_spread: 0.0,
            fireball_initial_angle_spread: 0.0,
            fireball_max_speed: f32::INFINITY,
            fireball_thruster_force: 0.0,
            fireball_homing_approach: 1.0,
            fireball_homing_target_spread: Vec2::ZERO,
            fireball_growth_rate: 0.0,
            fireball_initial_scale: 1.0,
            fireball_max_scale: 1.0,
        }
    }
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ProjectileAssets {
    #[asset(path = "image/projectile/missile.png")]
    missile_image: Handle<Image>,
    #[asset(path = "image/projectile/laser.png")]
    laser_image: Handle<Image>,
    #[asset(path = "image/projectile/fireball.png")]
    fireball_image: Handle<Image>,
}

impl Configure for ProjectileAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}
