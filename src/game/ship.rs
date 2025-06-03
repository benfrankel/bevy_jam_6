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
        ship(ship_assets.player_image.clone()),
        Health::new(100.0),
        IsPlayerShip,
        Collider::rectangle(85.0, 10.0),
        CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
        children![
            (IsWeapon, Transform::from_xyz(-10.0, 11.0, 0.0)),
            (IsWeapon, Transform::from_xyz(10.0, 11.0, 0.0)),
            (
                health_bar(vec2(100.0, 6.0)),
                Transform::from_xyz(0.0, -22.0, 0.1),
            ),
        ],
    )
}

pub fn enemy_ship(ship_assets: &ShipAssets) -> impl Bundle {
    (
        Name::new("EnemyShip"),
        ship(ship_assets.enemy_image.clone()),
        Health::new(100.0),
        IsEnemyShip,
        Collider::rectangle(167.0, 15.0),
        CollisionLayers::new(GameLayer::Enemy, LayerMask::ALL),
        children![(
            health_bar(vec2(200.0, 6.0)),
            Transform::from_xyz(0.0, 30.0, 0.1)
        )],
    )
}

fn ship(sprite: Handle<Image>) -> impl Bundle {
    (Sprite::from_image(sprite), RigidBody::Kinematic)
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
pub struct IsShip;

impl Configure for IsShip {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
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
