use crate::phase::Phase;
use crate::phase::helm::HelmActions;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.configure::<PhaseDisplay>();
}

pub(super) fn phase_display() -> impl Bundle {
    (
        Name::new("PhaseDisplay"),
        PhaseDisplay,
        ImageNode::default(),
        Node {
            width: Vw(9.1666),
            aspect_ratio: Some(1.0),
            ..Node::DEFAULT
        },
        InteractionSfx,
        Tooltip::fixed(Anchor::TopCenter, ""),
        Patch(|entity| {
            entity.observe(player_end_turn);
        }),
    )
}

fn player_end_turn(
    trigger: Trigger<Pointer<Click>>,
    mut player_actions: ResMut<ActionState<HelmActions>>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    player_actions.press(&HelmActions::EndTurn);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct PhaseDisplay;

impl Configure for PhaseDisplay {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(StateFlush, Phase::ANY.on_enter(sync_phase_display));
    }
}

fn sync_phase_display(
    phase: NextRef<Phase>,
    game_assets: Res<GameplayAssets>,
    mut phase_display_query: Query<(&mut ImageNode, &mut Tooltip), With<PhaseDisplay>>,
) {
    let phase = r!(phase.get());
    for (mut image_node, mut tooltip) in &mut phase_display_query {
        image_node.image = match phase {
            Phase::Setup => &game_assets.phase_setup,
            Phase::Helm => &game_assets.phase_player,
            Phase::Reactor | Phase::Player => &game_assets.phase_reactor,
            Phase::Enemy => &game_assets.phase_enemy,
        }
        .clone();
        tooltip.content = TooltipContent::Primary(
            RichText::from_sections(parse_rich(match phase {
                Phase::Setup => "[b]Storage phase[r]\n\nPulling modules from storage.",
                Phase::Helm => "[b]Player phase[r]\n\nClick here or press Space to end your turn.",
                Phase::Reactor | Phase::Player => {
                    "[b]Reactor phase[r]\n\nDirecting power to the reactor."
                },
                Phase::Enemy => "[b]Enemy phase[r]\n\nSustaining the enemy's barrage!",
            }))
            .with_justify(JustifyText::Center),
        );
    }
}
