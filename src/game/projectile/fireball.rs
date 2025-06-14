use core::f32;

use crate::game::GameAssets;
use crate::game::combat::damage::Damage;
use crate::game::combat::faction::Faction;
use crate::game::projectile::Growth;
use crate::game::projectile::Homing;
use crate::game::projectile::ProjectileConfig;
use crate::game::projectile::RotateWithVelocity;
use crate::game::projectile::Thruster;
use crate::game::projectile::projectile;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsFireball>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsFireball;

impl Configure for IsFireball {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn fireball(
    mut rng: impl Rng,
    projectile_config: &ProjectileConfig,
    game_assets: &GameAssets,
    faction: Faction,
    flux: f32,
    mut transform: Transform,
    target: Entity,
) -> impl Bundle {
    // Calculate initial transform.
    transform.translation += (projectile_config.fireball_initial_position
        + projectile_config.fireball_initial_position_spread
            * vec2(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)))
    .extend(0.0);
    transform.scale =
        (transform.scale.xy() * projectile_config.fireball_initial_scale).extend(transform.scale.z);
    let angle = transform.rotation.to_rot2().as_degrees()
        + projectile_config.fireball_initial_angle_spread * rng.gen_range(-1.0..=1.0);
    let angle = angle.to_radians();
    transform.rotation = Quat::radians(angle);

    // Calculate initial velocity.
    let speed = projectile_config.fireball_initial_speed
        + projectile_config.fireball_initial_speed_spread * rng.gen_range(-1.0..=1.0);
    let velocity = speed.max(1.0) * Vec2::from_angle(angle);

    // Calculate homing target position offset.
    let offset = projectile_config.fireball_homing_target_spread
        * vec2(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));

    (
        Name::new("Fireball"),
        IsFireball,
        projectile(faction, transform),
        Sprite::from_image(game_assets.fireball.clone()),
        Damage(projectile_config.fireball_damage * flux),
        Growth {
            rate: projectile_config.fireball_growth_rate,
            max_scale: projectile_config.fireball_max_scale,
        },
        Thruster {
            force: projectile_config.fireball_thruster_force,
        },
        Homing {
            target,
            offset,
            approach: projectile_config.fireball_homing_approach,
        },
        RotateWithVelocity,
        LinearVelocity(velocity),
        MaxLinearSpeed(projectile_config.fireball_max_speed),
        Collider::circle(5.0),
    )
}
