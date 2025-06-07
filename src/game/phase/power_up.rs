use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::game::GameAssets;
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
        Phase::PowerUp.on_enter(reset_step_timer_for_power_up),
    );
    app.add_systems(
        Update,
        Phase::PowerUp.on_update(
            step_power_up_phase
                .in_set(UpdateSystems::Update)
                .run_if(on_step_timer),
        ),
    );
}

fn reset_step_timer_for_power_up(
    phase_config: ConfigRef<PhaseConfig>,
    mut step_timer: ResMut<StepTimer>,
) {
    let phase_config = r!(phase_config.get());
    step_timer.0 = Timer::from_seconds(phase_config.power_up_first_cooldown, TimerMode::Once);
}

fn step_power_up_phase(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    audio_settings: Res<AudioSettings>,
    phase_config: ConfigRef<PhaseConfig>,
    mut phase: NextMut<Phase>,
    step: Res<Step>,
    mut step_timer: ResMut<StepTimer>,
    mut player_deck: ResMut<PlayerDeck>,
) {
    let phase_config = r!(phase_config.get());

    // Step powering up the reactor.
    if !player_deck.step_power_up() {
        phase.enter(Phase::Player);
        return;
    }
    commands.spawn(sfx_audio(
        &audio_settings,
        game_assets.module_activate_sfx.clone(),
    ));

    // Set the next cooldown.
    let cooldown = Duration::from_secs_f32(if player_deck.is_power_up_done() {
        phase_config.power_up_last_cooldown
    } else {
        phase_config.power_up_cooldown * phase_config.power_up_cooldown_decay.powf(step.0 as _)
    });
    step_timer.0.set_duration(cooldown);
}
