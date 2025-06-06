use crate::game::GameAssets;
use crate::game::phase::Phase;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Phase::ANY.on_enter(spawn_phase_display));
}

fn spawn_phase_display(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    phase: NextRef<Phase>,
) {
    commands.spawn((
        phase_display(&game_assets, r!(phase.get())),
        DespawnOnExitState::<Phase>::Recursive,
    ));
}

fn phase_display(game_assets: &GameAssets, phase: &Phase) -> impl Bundle {
    let description = match phase {
        Phase::Setup => "[b]Setup phase[r]\n\nPreparing the ship.",
        Phase::Player => "[b]Player phase[r]\n\nAwaiting your command.",
        Phase::Reactor => "[b]Reactor phase[r]\n\nDirecting power to the reactor.",
        Phase::Enemy => "[b]Enemy phase[r]\n\nSustaining the enemy's barrage.",
    };
    let image = match phase {
        Phase::Setup => &game_assets.phase_setup,
        Phase::Player => &game_assets.phase_player,
        Phase::Reactor => &game_assets.phase_reactor,
        Phase::Enemy => &game_assets.phase_enemy,
    }
    .clone();

    (
        Name::new("PhaseDisplay"),
        ImageNode::from(image),
        Node {
            left: Vw(27.0833),
            bottom: Vw(1.4583),
            width: Vw(9.1666),
            aspect_ratio: Some(1.0),
            ..Node::DEFAULT.abs()
        },
        Tooltip::fixed(Anchor::TopCenter, parse_rich(description)),
    )
}
