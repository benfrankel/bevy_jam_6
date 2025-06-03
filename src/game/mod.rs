pub mod combat;
pub mod deck;
pub mod hud;
pub mod level;
pub mod missile;
pub mod module;
pub mod ship;
pub mod turn;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        combat::plugin,
        deck::plugin,
        hud::plugin,
        level::plugin,
        missile::plugin,
        module::plugin,
        ship::plugin,
        turn::plugin,
    ));
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
}
