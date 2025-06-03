mod enemy;
mod player;
mod reactor;

use crate::game::level::Level;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<TurnConfig>, Turn, TurnTimer, Round)>();
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct TurnConfig {
    reactor_cooldown: f32,
    reactor_cooldown_decay: f32,
    reactor_first_cooldown: f32,
    reactor_last_cooldown: f32,

    enemy_cooldown: f32,
    enemy_cooldown_decay: f32,
    enemy_first_cooldown: f32,
    enemy_last_cooldown: f32,
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

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct TurnTimer(Timer);

impl Configure for TurnTimer {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(
            Update,
            Turn::ANY.on_update(
                tick_turn_timer
                    .in_set(UpdateSystems::TickTimers)
                    .in_set(PausableSystems),
            ),
        );
    }
}

fn tick_turn_timer(time: Res<Time>, mut turn_timer: ResMut<TurnTimer>) {
    turn_timer.0.tick(time.delta());
}

fn on_turn_timer(turn_timer: Res<TurnTimer>) -> bool {
    turn_timer.0.just_finished()
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
struct Round(usize);

impl Configure for Round {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(
            StateFlush,
            (
                (Turn::Enemy, Turn::Player).on_trans(increment_round),
                Turn::ANY.on_disable(reset_round),
            ),
        );
    }
}

fn increment_round(mut round: ResMut<Round>) {
    round.0 += 1;
}

fn reset_round(mut round: ResMut<Round>) {
    round.0 = 0;
}
