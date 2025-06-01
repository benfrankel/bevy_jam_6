pub mod draw_pile;
pub mod level;
pub mod missile;
pub mod module;
pub mod reactor;
pub mod ship;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        draw_pile::plugin,
        level::plugin,
        missile::plugin,
        module::plugin,
        reactor::plugin,
        ship::plugin,
    ));
}
