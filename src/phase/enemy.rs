use crate::animation::oscillate::Oscillate;
use crate::combat::death::Dead;
use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::deck::EnemyDeck;
use crate::level::Level;
use crate::menu::Menu;
use crate::module::OnAction;
use crate::phase::Phase;
use crate::phase::PhaseConfig;
use crate::phase::Round;
use crate::phase::Step;
use crate::phase::StepTimer;
use crate::phase::on_step_timer;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;
use crate::ship::EnemyShip;
use crate::ship::PlayerShip;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        Phase::Enemy.on_enter(reset_step_timer_for_enemy),
    );
    app.add_systems(
        Update,
        Phase::Enemy.on_update(
            step_enemy_phase
                .in_set(UpdateSystems::Update)
                .run_if(on_step_timer),
        ),
    );
}

fn reset_step_timer_for_enemy(
    round: Res<Round>,
    phase_config: ConfigRef<PhaseConfig>,
    mut step_timer: ResMut<StepTimer>,
    enemy_deck: Res<EnemyDeck>,
    enemy_is_dead: Single<Has<Dead>, With<EnemyShip>>,
) {
    if *enemy_is_dead || enemy_deck.is_done(round.0) {
        step_timer.0 = Timer::from_seconds(0.0, TimerMode::Once);
    } else {
        let phase_config = r!(phase_config.get());
        step_timer.0 = Timer::from_seconds(phase_config.enemy_first_cooldown, TimerMode::Once);
    }
}

fn step_enemy_phase(
    mut commands: Commands,
    phase_config: ConfigRef<PhaseConfig>,
    level: CurrentRef<Level>,
    round: Res<Round>,
    mut phase: NextMut<Phase>,
    step: Res<Step>,
    mut step_timer: ResMut<StepTimer>,
    mut enemy_deck: ResMut<EnemyDeck>,
    mut enemy_ship: Single<
        (Entity, Has<Dead>, &mut ExternalForce, &mut Oscillate),
        With<EnemyShip>,
    >,
    player_ship: Single<Entity, With<PlayerShip>>,
    audio_settings: Res<AudioSettings>,
    game_assets: Res<GameplayAssets>,
    mut menu: ResMut<NextStateStack<Menu>>,
) {
    let phase_config = r!(phase_config.get());

    // Explode or flee if dead.
    if enemy_ship.1 {
        if level.is_in(&Level(9)) {
            // Death is ugly. This code is a metaphor for the realities of death.
            let death_oscillate1 = Oscillate::new(vec2(0.0, 0.0), vec2(1.34, 0.0), vec2(1.1, 1.2));
            let death_oscillate2 =
                Oscillate::new(vec2(2.0, 2.0), vec2(1.34, 0.0), vec2(31.3, 46.7));
            if enemy_ship.3.rate != death_oscillate1.rate
                && enemy_ship.3.rate != death_oscillate2.rate
            {
                *enemy_ship.3 = death_oscillate1;
                step_timer.0.set_duration(Duration::from_secs_f32(1.0));
            } else if enemy_ship.3.rate != death_oscillate2.rate {
                *enemy_ship.3 = death_oscillate2;
                step_timer.0.set_duration(Duration::from_secs_f32(1.5));
            } else {
                commands.entity(enemy_ship.0).try_despawn();
                commands.spawn((
                    sfx_audio(&audio_settings, game_assets.ship_death_sfx.clone(), 1.0),
                    DespawnOnExitState::<Level>::default(),
                ));

                menu.push(Menu::Victory);
                menu.acquire();
            }
        } else {
            enemy_ship.2.apply_force(phase_config.enemy_escape_force);
            step_timer.0.set_duration(Duration::from_secs_f32(30.0));
        }
        return;
    }

    // Step the enemy deck.
    let Some(action) = enemy_deck.step(round.0) else {
        phase.enter(Phase::Setup);
        return;
    };
    commands.trigger(OnAction {
        action,
        source: enemy_ship.0,
        target: *player_ship,
    });

    // Set the next cooldown.
    let cooldown = Duration::from_secs_f32(if enemy_deck.is_done(round.0) {
        phase_config.enemy_last_cooldown
    } else {
        phase_config.enemy_cooldown * phase_config.enemy_cooldown_decay.powi(step.0 as _)
    });
    step_timer.0.set_duration(cooldown);
}
