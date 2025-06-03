mod enemy;
mod player;
mod reactor;

use crate::game::level::Level;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<TurnConfig>, Turn)>();
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct TurnConfig {
    reactor_cooldown: f32,
    reactor_cooldown_decay: f32,
    reactor_first_cooldown: f32,
    reactor_last_cooldown: f32,
}

impl Config for TurnConfig {
    const FILE: &'static str = "turn.ron";
}

#[derive(State, Reflect, Copy, Clone, Default, Eq, PartialEq, Debug)]
#[state(log_flush, after(Level))]
#[reflect(Resource)]
pub enum Turn {
    #[default]
    Player,
    Reactor,
    Enemy,
}

impl Configure for Turn {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(
            StateFlush,
            (Level::ANY.on_edge(Turn::disable, (Turn::enter_default, Turn::trigger)),),
        );

        app.add_plugins((player::plugin, reactor::plugin, enemy::plugin));
    }
}
