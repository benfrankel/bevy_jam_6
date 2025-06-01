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

pub fn player_ship(ship_assets: &ShipAssets) -> impl Bundle {
    ship(ship_assets.player_image.clone())
}

pub fn enemy_ship(ship_assets: &ShipAssets) -> impl Bundle {
    ship(ship_assets.enemy_image.clone())
}

fn ship(sprite: Handle<Image>) -> impl Bundle {
    (Name::new("Ship"), Sprite::from_image(sprite))
}
