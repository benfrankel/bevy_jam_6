use crate::game::deck::EnemyDeck;
use crate::game::module::OnModuleAction;
use crate::game::ship::IsEnemyShip;
use crate::game::turn::Turn;
use crate::game::turn::TurnConfig;
use crate::game::turn::TurnTimer;
use crate::game::turn::on_turn_timer;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Turn::Enemy.on_enter(reset_turn_timer_for_enemy));
    app.add_systems(
        Update,
        Turn::Enemy.on_update(step_enemy_turn.run_if(on_turn_timer)),
    );
}

fn reset_turn_timer_for_enemy(
    turn_config: ConfigRef<TurnConfig>,
    mut turn_timer: ResMut<TurnTimer>,
) {
    let turn_config = r!(turn_config.get());
    turn_timer.0 = Timer::from_seconds(turn_config.enemy_first_cooldown, TimerMode::Once);
}

fn step_enemy_turn(
    mut commands: Commands,
    turn_config: ConfigRef<TurnConfig>,
    mut next_turn: NextMut<Turn>,
    mut turn_timer: ResMut<TurnTimer>,
    mut enemy_deck: ResMut<EnemyDeck>,
    enemy_ship: Single<Entity, With<IsEnemyShip>>,
) {
    let turn_config = r!(turn_config.get());

    // Step the enemy deck.
    let Some(action) = enemy_deck.next() else {
        next_turn.enter(Turn::Player);
        return;
    };
    commands.entity(*enemy_ship).trigger(OnModuleAction(action));

    // Set the next cooldown.
    let cooldown = Duration::from_secs_f32(if enemy_deck.is_done() {
        turn_config.enemy_last_cooldown
    } else {
        turn_config.enemy_cooldown * turn_config.enemy_cooldown_decay.powf(1.0)
    });
    turn_timer.0.set_duration(cooldown);
    turn_timer.0.reset();
}
