use crate::game::deck::PlayerDeck;
use crate::game::phase::Phase;
use crate::game::phase::PhaseConfig;
use crate::game::phase::Step;
use crate::game::phase::StepTimer;
use crate::game::phase::on_step_timer;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        Phase::Setup.on_enter(reset_step_timer_for_setup),
    );
    app.add_systems(
        Update,
        Phase::Setup.on_update(
            step_setup_phase
                .in_set(UpdateSystems::Update)
                .run_if(on_step_timer),
        ),
    );
}

fn reset_step_timer_for_setup(
    phase_config: ConfigRef<PhaseConfig>,
    mut step_timer: ResMut<StepTimer>,
) {
    let phase_config = r!(phase_config.get());
    step_timer.0 = Timer::from_seconds(phase_config.setup_first_cooldown, TimerMode::Once);
}

fn step_setup_phase(
    phase_config: ConfigRef<PhaseConfig>,
    mut next_phase: NextMut<Phase>,
    step: Res<Step>,
    mut step_timer: ResMut<StepTimer>,
    mut player_deck: ResMut<PlayerDeck>,
) {
    let phase_config = r!(phase_config.get());

    // Step the setup.
    if !player_deck.step_setup() {
        next_phase.enter(Phase::Player);
        return;
    }

    // Set the next cooldown.
    let cooldown = Duration::from_secs_f32(if player_deck.is_setup_done() {
        phase_config.setup_last_cooldown
    } else {
        phase_config.setup_cooldown * phase_config.setup_cooldown_decay.powi(step.0 as _)
    });
    step_timer.0.set_duration(cooldown);
    step_timer.0.reset();
}
