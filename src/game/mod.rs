pub mod combat;
pub mod deck;
pub mod hud;
pub mod level;
pub mod missile;
pub mod module;
pub mod phase;
pub mod ship;

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
        phase::plugin,
    ));
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
}
