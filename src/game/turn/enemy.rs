use crate::game::module::ModuleAction;
use crate::game::module::OnModuleAction;
use crate::game::ship::IsEnemyShip;
use crate::game::turn::Turn;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, Turn::Enemy.on_update(fire_enemy_missile));
}

// TODO: Flesh out enemy turn.
fn fire_enemy_missile(
    mut commands: Commands,
    mut next_turn: NextMut<Turn>,
    enemy_ship: Single<Entity, With<IsEnemyShip>>,
) {
    commands
        .entity(*enemy_ship)
        .trigger(OnModuleAction(ModuleAction::Missile));

    next_turn.enter(Turn::Player);
}
