use crate::game::deck::EnemyDeck;
use crate::game::module::OnModuleAction;
use crate::game::phase::Phase;
use crate::game::phase::PhaseConfig;
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
    phase_config: ConfigRef<PhaseConfig>,
    mut step_timer: ResMut<StepTimer>,
) {
    let phase_config = r!(phase_config.get());
    step_timer.0 = Timer::from_seconds(phase_config.enemy_first_cooldown, TimerMode::Once);
}

fn step_enemy_phase(
    mut commands: Commands,
    phase_config: ConfigRef<PhaseConfig>,
    mut next_phase: NextMut<Phase>,
    step: Res<Step>,
    mut step_timer: ResMut<StepTimer>,
    mut enemy_deck: ResMut<EnemyDeck>,
    enemy_ship: Single<Entity, With<IsEnemyShip>>,
) {
    let phase_config = r!(phase_config.get());

    // Step the enemy deck.
    let Some(action) = enemy_deck.step() else {
        next_phase.enter(Phase::Setup);
        return;
    };
    commands.entity(*enemy_ship).trigger(OnModuleAction(action));

    // Set the next cooldown.
    let cooldown = Duration::from_secs_f32(if enemy_deck.is_done() {
        phase_config.enemy_last_cooldown
    } else {
        phase_config.enemy_cooldown * phase_config.enemy_cooldown_decay.powi(step.0 as _)
    });
    step_timer.0.set_duration(cooldown);
    step_timer.0.reset();
}
