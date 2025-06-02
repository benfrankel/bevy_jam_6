pub mod flux;
pub mod hud;
pub mod level;
pub mod missile;
pub mod ship;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        flux::plugin,
        hud::plugin,
        level::plugin,
        missile::plugin,
        ship::plugin,
    ));
}
