use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ShipAssets>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ShipAssets {
    #[asset(path = "image/ship/player.png")]
    player_image: Handle<Image>,
    #[asset(path = "image/ship/enemy.png")]
    enemy_image: Handle<Image>,
}

impl Configure for ShipAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Component, Debug)]
pub struct PlayerShip;

#[derive(Component)]
pub struct EnemyShip;

pub fn player_ship(ship_assets: &ShipAssets) -> impl Bundle {
    (
        Name::new("PlayerShip"),
        PlayerShip,
        ship(ship_assets.player_image.clone()),
        RigidBody::Kinematic,
        Collider::rectangle(85., 10.),
    )
}

pub fn enemy_ship(ship_assets: &ShipAssets) -> impl Bundle {
    (
        Name::new("EnemyShip"),
        EnemyShip,
        ship(ship_assets.enemy_image.clone()),
        RigidBody::Kinematic,
        Collider::rectangle(167., 15.),
    )
}

fn ship(sprite: Handle<Image>) -> impl Bundle {
    Sprite::from_image(sprite)
}
