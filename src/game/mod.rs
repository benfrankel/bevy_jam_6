pub mod deck;
pub mod health;
pub mod hud;
pub mod level;
pub mod missile;
pub mod ship;
pub mod turn;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        deck::plugin,
        health::plugin,
        hud::plugin,
        level::plugin,
        missile::plugin,
        ship::plugin,
        turn::plugin,
    ));
}

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
}
