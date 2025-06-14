use crate::game::deck::PlayerDeck;
use crate::game::module::OnModuleAction;
use crate::game::phase::Phase;
use crate::game::phase::PhaseConfig;
use crate::game::phase::Step;
use crate::game::phase::StepTimer;
use crate::game::phase::on_step_timer;
use crate::game::ship::EnemyShip;
use crate::game::ship::PlayerShip;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        Phase::Player.on_enter(reset_step_timer_for_player),
    );
    app.add_systems(
        Update,
        Phase::Player.on_update(
            step_player_phase
                .in_set(UpdateSystems::Update)
                .run_if(on_step_timer),
        ),
    );
}

fn reset_step_timer_for_player(
    phase_config: ConfigRef<PhaseConfig>,
    mut step_timer: ResMut<StepTimer>,
    player_deck: Res<PlayerDeck>,
) {
    if player_deck.is_player_done() {
        step_timer.0 = Timer::from_seconds(0.0, TimerMode::Once);
    } else {
        let phase_config = r!(phase_config.get());
        step_timer.0 = Timer::from_seconds(phase_config.player_first_cooldown, TimerMode::Once);
    }
}

fn step_player_phase(
    mut commands: Commands,
    phase_config: ConfigRef<PhaseConfig>,
    mut phase: NextMut<Phase>,
    step: Res<Step>,
    mut step_timer: ResMut<StepTimer>,
    mut player_deck: ResMut<PlayerDeck>,
    player_ship: Single<Entity, With<PlayerShip>>,
    enemy_ship: Single<Entity, With<EnemyShip>>,
) {
    let phase_config = r!(phase_config.get());

    // Step through the player reactor chain.
    let Some(action) = player_deck.step_player() else {
        phase.enter(Phase::Enemy);
        return;
    };
    commands.trigger(OnModuleAction {
        action,
        source: *player_ship,
        target: *enemy_ship,
    });

    // Set the next cooldown.
    let cooldown = Duration::from_secs_f32(if player_deck.is_player_done() {
        phase_config.player_last_cooldown
    } else {
        phase_config.player_cooldown * phase_config.player_cooldown_decay.powi(step.0 as _)
    });
    step_timer.0.set_duration(cooldown);
}
