pub mod damage;
pub mod faction;
pub mod health;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((damage::plugin, faction::plugin, health::plugin));
}
