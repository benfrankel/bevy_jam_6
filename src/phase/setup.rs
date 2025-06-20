use crate::animation::oscillate::Oscillate;
use crate::combat::death::Dead;
use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::deck::PlayerDeck;
use crate::level::Level;
use crate::menu::Menu;
use crate::phase::Phase;
use crate::phase::PhaseConfig;
use crate::phase::Step;
use crate::phase::StepTimer;
use crate::phase::on_step_timer;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;
use crate::ship::PlayerShip;

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
    player_deck: Res<PlayerDeck>,
) {
    let phase_config = r!(phase_config.get());
    step_timer.0 = Timer::from_seconds(
        if player_deck.is_setup_done() {
            phase_config.setup_skip_cooldown
        } else {
            phase_config.setup_first_cooldown
        },
        TimerMode::Once,
    );
}

fn step_setup_phase(
    mut commands: Commands,
    phase_config: ConfigRef<PhaseConfig>,
    mut phase: NextMut<Phase>,
    step: Res<Step>,
    mut step_timer: ResMut<StepTimer>,
    mut player_deck: ResMut<PlayerDeck>,
    mut player_ship: Single<(Entity, Has<Dead>, &mut Oscillate), With<PlayerShip>>,
    audio_settings: Res<AudioSettings>,
    game_assets: Res<GameplayAssets>,
    mut menu: ResMut<NextStateStack<Menu>>,
) {
    let phase_config = r!(phase_config.get());

    // Explode if dead.
    if player_ship.1 {
        // Death is ugly. This code is a metaphor for the realities of death.
        let death_oscillate1 = Oscillate::new(vec2(0.0, 0.0), vec2(1.34, 0.0), vec2(1.1, 1.2));
        let death_oscillate2 = Oscillate::new(vec2(2.0, 2.0), vec2(1.34, 0.0), vec2(31.3, 46.7));
        if player_ship.2.rate != death_oscillate1.rate
            && player_ship.2.rate != death_oscillate2.rate
        {
            *player_ship.2 = death_oscillate1;
            step_timer.0.set_duration(Duration::from_secs_f32(1.0));
        } else if player_ship.2.rate != death_oscillate2.rate {
            *player_ship.2 = death_oscillate2;
            step_timer.0.set_duration(Duration::from_secs_f32(1.5));
        } else {
            commands.entity(player_ship.0).try_despawn();
            commands.spawn((
                sfx_audio(&audio_settings, game_assets.ship_death_sfx.clone(), 1.0),
                DespawnOnExitState::<Level>::default(),
            ));

            menu.push(Menu::Defeat);
            menu.acquire();
        }

        return;
    }

    // Step the setup.
    if !player_deck.step_setup(&mut thread_rng()) {
        phase.enter(Phase::Helm);
        return;
    }

    // Set the next cooldown.
    let cooldown = Duration::from_secs_f32(if player_deck.is_setup_done() {
        phase_config.setup_last_cooldown
    } else {
        phase_config.setup_cooldown.sample(step.0 as _)
    });
    step_timer.0.set_duration(cooldown);
}
