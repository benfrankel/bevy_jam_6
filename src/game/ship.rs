use crate::game::GameLayer;
use crate::game::health::Health;
use crate::game::health::health_bar;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ShipAssets, IsPlayerShip, IsEnemyShip, IsWeapon)>();
}

pub fn player_ship(ship_assets: &ShipAssets) -> impl Bundle {
    (
        Name::new("PlayerShip"),
        IsPlayerShip,
        ship(ship_assets.player_image.clone()),
        Health::new(100.0),
        Collider::rectangle(85.0, 10.0),
        CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
        children![
            (weapon(), Transform::from_xyz(-10.0, 0.0, -0.1)),
            (weapon(), Transform::from_xyz(10.0, 0.0, -0.1)),
            (health_bar(100.0, 6.0), Transform::from_xyz(0.0, -22.0, 0.1)),
        ],
    )
}

pub fn enemy_ship(ship_assets: &ShipAssets) -> impl Bundle {
    (
        Name::new("EnemyShip"),
        IsEnemyShip,
        ship(ship_assets.enemy_image.clone()),
        Health::new(100.0),
        Collider::rectangle(167.0, 15.0),
        CollisionLayers::new(GameLayer::Enemy, LayerMask::ALL),
        children![
            (
                weapon(),
                Transform::from_xyz(-43.0, 0.0, -0.1).with_rotation(Rot2::PI.to_quat()),
            ),
            (
                weapon(),
                Transform::from_xyz(-27.0, 0.0, -0.1).with_rotation(Rot2::PI.to_quat()),
            ),
            (
                weapon(),
                Transform::from_xyz(38.0, 0.0, -0.1).with_rotation(Rot2::PI.to_quat()),
            ),
            (health_bar(200.0, 6.0), Transform::from_xyz(0.0, 30.0, 0.1)),
        ],
    )
}

fn ship(sprite: Handle<Image>) -> impl Bundle {
    (Sprite::from_image(sprite), RigidBody::Kinematic)
}

fn weapon() -> impl Bundle {
    (
        Name::new("Weapon"),
        IsWeapon,
        #[cfg(feature = "dev")]
        Collider::triangle(vec2(-2.0, 0.0), vec2(2.0, 0.0), vec2(0.0, 8.0)),
    )
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

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsPlayerShip;

impl Configure for IsPlayerShip {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsEnemyShip;

impl Configure for IsEnemyShip {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsWeapon;

impl Configure for IsWeapon {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
