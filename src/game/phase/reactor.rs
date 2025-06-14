use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::game::GameAssets;
use crate::game::deck::PlayerDeck;
use crate::game::level::Level;
use crate::game::phase::Phase;
use crate::game::phase::PhaseConfig;
use crate::game::phase::StepTimer;
use crate::game::phase::on_step_timer;
use crate::game::stats::Stats;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        Phase::Reactor.on_enter(reset_step_timer_for_power_up),
    );
    app.add_systems(
        Update,
        Phase::Reactor.on_update(
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
    step_timer.0 = Timer::from_seconds(phase_config.reactor_first_cooldown, TimerMode::Once);
}

fn step_power_up_phase(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    audio_settings: Res<AudioSettings>,
    phase_config: ConfigRef<PhaseConfig>,
    mut phase: NextMut<Phase>,
    mut step_timer: ResMut<StepTimer>,
    mut player_deck: ResMut<PlayerDeck>,
    mut stats: ResMut<Stats>,
) {
    let phase_config = r!(phase_config.get());

    // Step powering up the reactor.
    if !player_deck.step_reactor() {
        phase.enter(Phase::Player);
        return;
    }
    commands.spawn((
        sfx_audio(
            &audio_settings,
            game_assets.module_activate_sfx.clone(),
            2f32.powf((player_deck.chain - 1.0) / phase_config.reactor_sfx_tones),
        ),
        DespawnOnExitState::<Level>::default(),
    ));

    // Record max flux.
    stats.highest_flux = stats.highest_flux.max(player_deck.flux);

    // Set the next cooldown.
    let cooldown = Duration::from_secs_f32(if player_deck.is_reactor_done() {
        phase_config.reactor_last_cooldown
    } else {
        phase_config.reactor_cooldown
            * phase_config
                .reactor_cooldown_decay
                .powf(player_deck.chain - 1.0)
    });
    step_timer.0.set_duration(cooldown);
}
