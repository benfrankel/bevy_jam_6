use crate::game::deck::PlayerDeck;
use crate::game::module::OnModuleAction;
use crate::game::ship::IsPlayerShip;
use crate::game::turn::Turn;
use crate::game::turn::TurnConfig;
use crate::game::turn::TurnTimer;
use crate::game::turn::on_turn_timer;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        Turn::Reactor.on_enter(reset_turn_timer_for_reactor),
    );
    app.add_systems(
        Update,
        Turn::Reactor.on_update(step_reactor_turn.run_if(on_turn_timer)),
    );
}

fn reset_turn_timer_for_reactor(
    turn_config: ConfigRef<TurnConfig>,
    mut turn_timer: ResMut<TurnTimer>,
) {
    let turn_config = r!(turn_config.get());
    turn_timer.0 = Timer::from_seconds(turn_config.reactor_first_cooldown, TimerMode::Once);
}

fn step_reactor_turn(
    mut commands: Commands,
    turn_config: ConfigRef<TurnConfig>,
    mut next_turn: NextMut<Turn>,
    mut turn_timer: ResMut<TurnTimer>,
    mut player_deck: ResMut<PlayerDeck>,
    player_ship: Single<Entity, With<IsPlayerShip>>,
) {
    let turn_config = r!(turn_config.get());

    // Step the player deck.
    let Some(action) = player_deck.next() else {
        next_turn.enter(Turn::Enemy);
        return;
    };
    commands
        .entity(*player_ship)
        .trigger(OnModuleAction(action));

    // Set the next cooldown.
    let cooldown = Duration::from_secs_f32(if player_deck.is_done() {
        turn_config.reactor_last_cooldown
    } else {
        turn_config.reactor_cooldown
            * turn_config
                .reactor_cooldown_decay
                .powf(player_deck.flux - 1.0)
    });
    turn_timer.0.set_duration(cooldown);
    turn_timer.0.reset();
}
