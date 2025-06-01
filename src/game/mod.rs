pub mod draw_pile;
pub mod level;
pub mod missile;
pub mod module;
pub mod reactor;
pub mod ship;
pub mod stage;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        draw_pile::plugin,
        level::plugin,
        missile::plugin,
        module::plugin,
        reactor::plugin,
        ship::plugin,
        stage::plugin,
    ));
}
