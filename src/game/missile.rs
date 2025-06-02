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

impl Configure for MissileAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

pub fn missile(missile_assets: &MissileAssets) -> impl Bundle {
    (
        Name::new("Missile"),
        Sprite::from_image(missile_assets.image.clone()),
        RigidBody::Dynamic,
        Collider::capsule(5., 10.),
        LinearVelocity::ZERO,
        Missile,
    )
}

pub fn start_position() -> Vec3 {
    let player_launch_pos = -46. + 15.;
    Vec3::new(61., player_launch_pos, 0.)
}
