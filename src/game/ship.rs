use crate::game::GameLayer;
use crate::game::combat::faction::Faction;
use crate::game::combat::health::Health;
use crate::game::combat::health::health_bar;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<ShipConfig>,
        ShipAssets,
        IsPlayerShip,
        IsEnemyShip,
        IsWeapon,
    )>();
}

pub fn player_ship(ship_config: &ShipConfig, ship_assets: &ShipAssets) -> impl Bundle {
    let weapons = ship_config.player_weapons.clone();
    let health_bar_transform =
        Transform::from_translation(ship_config.player_health_bar_offset.extend(0.1))
            .with_scale(ship_config.player_health_bar_size.extend(1.0));

    (
        Name::new("PlayerShip"),
        IsPlayerShip,
        Faction::Player,
        ship(ship_assets.player_image.clone()),
        Health::new(100.0),
        Collider::rectangle(85.0, 10.0),
        CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent.spawn((health_bar(), health_bar_transform));

            let rotation = Rot2::turn_fraction(0.25).to_quat();
            for pos in weapons {
                parent.spawn((
                    weapon(),
                    Transform::from_translation(pos.extend(-0.1)).with_rotation(rotation),
                ));
            }
        })),
    )
}

pub fn enemy_ship(ship_config: &ShipConfig, ship_assets: &ShipAssets) -> impl Bundle {
    let weapons = ship_config.player_weapons.clone();
    let health_bar_transform =
        Transform::from_translation(ship_config.enemy_health_bar_offset.extend(0.1))
            .with_scale(ship_config.enemy_health_bar_size.extend(1.0));

    (
        Name::new("EnemyShip"),
        IsEnemyShip,
        Faction::Enemy,
        ship(ship_assets.enemy_image.clone()),
        Health::new(100.0),
        Collider::rectangle(167.0, 15.0),
        CollisionLayers::new(GameLayer::Enemy, LayerMask::ALL),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent.spawn((health_bar(), health_bar_transform));

            let rotation = Rot2::turn_fraction(0.75).to_quat();
            for pos in weapons {
                parent.spawn((
                    weapon(),
                    Transform::from_translation(pos.extend(-0.1)).with_rotation(rotation),
                ));
            }
        })),
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
        Collider::triangle(vec2(0.0, -2.0), vec2(0.0, 2.0), vec2(8.0, 0.0)),
    )
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct ShipConfig {
    player_weapons: Vec<Vec2>,
    player_health_bar_offset: Vec2,
    player_health_bar_size: Vec2,

    enemy_weapons: Vec<Vec2>,
    enemy_health_bar_offset: Vec2,
    enemy_health_bar_size: Vec2,
}

impl Config for ShipConfig {
    const FILE: &'static str = "ship.ron";
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
