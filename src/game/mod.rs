pub mod deck;
pub mod health;
pub mod hud;
pub mod level;
pub mod missile;
pub mod ship;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        deck::plugin,
        hud::plugin,
        level::plugin,
        missile::plugin,
        ship::plugin,
    ));
}

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
}
