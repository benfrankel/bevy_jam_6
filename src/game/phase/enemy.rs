use crate::game::combat::death::IsDead;
use crate::game::deck::EnemyDeck;
use crate::game::module::OnModuleAction;
use crate::game::phase::Phase;
use crate::game::phase::PhaseConfig;
use crate::game::phase::Round;
use crate::game::phase::Step;
use crate::game::phase::StepTimer;
use crate::game::phase::on_step_timer;
use crate::game::ship::IsEnemyShip;
use crate::prelude::*;

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
    enemy_is_dead: Single<Has<IsDead>, With<IsEnemyShip>>,
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
    round: Res<Round>,
    phase_config: ConfigRef<PhaseConfig>,
    mut phase: NextMut<Phase>,
    step: Res<Step>,
    mut step_timer: ResMut<StepTimer>,
    mut enemy_deck: ResMut<EnemyDeck>,
    mut enemy_ship: Single<(Entity, Has<IsDead>, &mut ExternalForce), With<IsEnemyShip>>,
) {
    let phase_config = r!(phase_config.get());

    if enemy_ship.1 {
        enemy_ship.2.apply_force(phase_config.enemy_escape_force);
        return;
    }

    // Step the enemy deck.
    let Some(action) = enemy_deck.step(round.0) else {
        phase.enter(Phase::Setup);
        return;
    };
    commands
        .entity(enemy_ship.0)
        .trigger(OnModuleAction(action));

    // Set the next cooldown.
    let cooldown = Duration::from_secs_f32(if enemy_deck.is_done(round.0) {
        phase_config.enemy_last_cooldown
    } else {
        phase_config.enemy_cooldown * phase_config.enemy_cooldown_decay.powi(step.0 as _)
    });
    step_timer.0.set_duration(cooldown);
}
