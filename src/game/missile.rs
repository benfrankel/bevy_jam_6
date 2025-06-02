use crate::game::level::{spawn_level, Level};
use crate::game::ship::PlayerShip;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<MissileAssets>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct MissileAssets {
    #[asset(path = "image/ship/missile.png")]
    image: Handle<Image>,
}

#[derive(Component)]
pub struct Missile;

#[derive(Component, Debug)]
pub struct IsMissileLauncher;

impl Configure for MissileAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
        app.add_systems(StateFlush, Level::ANY.on_enter(spawn_missile).after(spawn_level));
        app.add_systems(Update, Level::ANY.on_update(launch_missile));
    }
}

fn spawn_missile(
    mut commands: Commands,
    missile_assets: Res<MissileAssets>,
    launchers_query: Query<(&Transform, &IsMissileLauncher)>,
    // player_ship: Query<&PlayerShip>,
) {
    let n = thread_rng().gen_range(0..1);
    let launcher = launchers_query.iter().collect::<Vec<_>>();
    commands.spawn(missile(&missile_assets, launcher[n]));
}

pub fn missile(missile_assets: &MissileAssets, launcher: (&Transform, &IsMissileLauncher)) -> impl Bundle {
    (
        Name::new("Missile"),
        Missile,
        Sprite::from_image(missile_assets.image.clone()),
        RigidBody::Dynamic,
        Collider::capsule(3., 7.),
        LinearVelocity::ZERO,
        DespawnOnExitState::<Level>::default(),
        Transform::from_translation(start_position(61.)),
    )
}

fn launch_missile(query: Single<&mut LinearVelocity, With<Missile>>) {
    let mut velocity = query;
    velocity.y += 1.;
}

pub fn start_position(x_pos: f32) -> Vec3 {
    Vec3::new(x_pos, -46., 0.)
}
