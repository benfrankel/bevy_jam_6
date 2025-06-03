use crate::game::deck::Deck;
use crate::game::module::ModuleAction;
use crate::game::module::OnModuleAction;
use crate::game::ship::IsPlayerShip;
use crate::game::turn::Turn;
use crate::game::turn::TurnConfig;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ReactorTimer>();

    app.add_systems(
        Update,
        Turn::Reactor.on_update(step_reactor.run_if(on_reactor_timer)),
    );
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
struct ReactorTimer(Timer);

impl Configure for ReactorTimer {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(StateFlush, Turn::Reactor.on_enter(reset_reactor_timer));
        app.add_systems(
            Update,
            Turn::Reactor.on_update(tick_reactor_timer.in_set(UpdateSystems::TickTimers)),
        );
    }
}

impl Default for ReactorTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Once))
    }
}

fn reset_reactor_timer(mut reactor_timer: ResMut<ReactorTimer>) {
    *reactor_timer = default();
}

fn tick_reactor_timer(time: Res<Time>, mut reactor_timer: ResMut<ReactorTimer>) {
    reactor_timer.0.tick(time.delta());
}

fn on_reactor_timer(reactor_timer: Res<ReactorTimer>) -> bool {
    reactor_timer.0.just_finished()
}

fn step_reactor(
    mut commands: Commands,
    turn_config: ConfigRef<TurnConfig>,
    mut next_turn: NextMut<Turn>,
    mut deck: ResMut<Deck>,
    mut reactor_timer: ResMut<ReactorTimer>,
    player_ship: Single<Entity, With<IsPlayerShip>>,
) {
    let turn_config = r!(turn_config.get());

    // Step the reactor.
    deck.step_reactor();
    if matches!(deck.last_effect, ModuleAction::Nothing) {
        next_turn.enter(Turn::Enemy);
        return;
    }
    commands
        .entity(*player_ship)
        .trigger(OnModuleAction(deck.last_effect));

    // Reset the reactor timer.
    let delay = if deck.is_reactor_done() {
        turn_config.reactor_final_cooldown
    } else {
        turn_config.reactor_cooldown_decay.powf(deck.flux - 1.0)
    };
    reactor_timer.0.set_duration(Duration::from_secs_f32(delay));
    reactor_timer.0.reset();
}
