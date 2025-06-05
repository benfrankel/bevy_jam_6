use crate::game::hud::HudAssets;
use crate::game::phase::Phase;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Phase::ANY.on_enter(spawn_phase_display));
}

fn spawn_phase_display(mut commands: Commands, hud_assets: Res<HudAssets>, phase: NextRef<Phase>) {
    commands.spawn((
        phase_display(&hud_assets, r!(phase.get())),
        DespawnOnExitState::<Phase>::Recursive,
    ));
}

fn phase_display(hud_assets: &HudAssets, phase: &Phase) -> impl Bundle {
    let description = match phase {
        Phase::Setup => "[b]Setup phase[r]\n\nPreparing the ship.",
        Phase::Player => "[b]Player phase[r]\n\nAwaiting your command.",
        Phase::Reactor => "[b]Reactor phase[r]\n\nDirecting power to the reactor.",
        Phase::Enemy => "[b]Enemy phase[r]\n\nSustaining the enemy's barrage.",
    };
    let image = match phase {
        Phase::Setup => &hud_assets.phase_setup,
        Phase::Player => &hud_assets.phase_player,
        Phase::Reactor => &hud_assets.phase_reactor,
        Phase::Enemy => &hud_assets.phase_enemy,
    }
    .clone();

    (
        Name::new("PhaseDisplay"),
        ImageNode::from(image),
        Node {
            top: Vw(2.7083),
            right: Vw(2.7083),
            width: Vw(8.75),
            aspect_ratio: Some(1.0),
            ..Node::DEFAULT.abs()
        },
        Tooltip::fixed(Anchor::CenterLeft, parse_rich(description)),
    )
}
