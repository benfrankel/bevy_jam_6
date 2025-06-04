use crate::game::deck::PlayerDeck;
use crate::game::module::OnModuleAction;
use crate::game::phase::Phase;
use crate::game::phase::PhaseConfig;
use crate::game::phase::StepTimer;
use crate::game::phase::on_step_timer;
use crate::game::ship::IsPlayerShip;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        Phase::Reactor.on_enter(reset_step_timer_for_reactor),
    );
    app.add_systems(
        Update,
        Phase::Reactor.on_update(
            step_reactor_phase
                .in_set(UpdateSystems::Update)
                .run_if(on_step_timer),
        ),
    );
}

fn reset_step_timer_for_reactor(
    phase_config: ConfigRef<PhaseConfig>,
    mut step_timer: ResMut<StepTimer>,
) {
    let phase_config = r!(phase_config.get());
    step_timer.0 = Timer::from_seconds(phase_config.reactor_first_cooldown, TimerMode::Once);
}

fn step_reactor_phase(
    mut commands: Commands,
    phase_config: ConfigRef<PhaseConfig>,
    mut next_phase: NextMut<Phase>,
    mut step_timer: ResMut<StepTimer>,
    mut player_deck: ResMut<PlayerDeck>,
    player_ship: Single<Entity, With<IsPlayerShip>>,
) {
    let phase_config = r!(phase_config.get());

    // Step the player deck.
    let Some(action) = player_deck.step_reactor() else {
        next_phase.enter(Phase::Enemy);
        return;
    };
    commands
        .entity(*player_ship)
        .trigger(OnModuleAction(action));

    // Set the next cooldown.
    let cooldown = Duration::from_secs_f32(if player_deck.is_reactor_done() {
        phase_config.reactor_last_cooldown
    } else {
        phase_config.reactor_cooldown
            * phase_config
                .reactor_cooldown_decay
                .powf(player_deck.flux - 1.0)
    });
    step_timer.0.set_duration(cooldown);
    step_timer.0.reset();
}
