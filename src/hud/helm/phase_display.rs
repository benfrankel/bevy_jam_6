use crate::deck::PlayerDeck;
use crate::module::ModuleStatus;
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
        InteractionDisabled(false),
        BoxShadow::default(),
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
pub struct PhaseDisplay;

impl Configure for PhaseDisplay {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(StateFlush, Phase::ANY.on_enter(sync_phase_display));
        app.add_systems(
            Update,
            sync_phase_display_glow.in_set(UpdateSystems::SyncLate),
        );
    }
}

fn sync_phase_display(
    phase: NextRef<Phase>,
    game_assets: Res<GameplayAssets>,
    mut phase_display_query: Query<
        (&mut ImageNode, &mut Tooltip, &mut InteractionDisabled),
        With<PhaseDisplay>,
    >,
) {
    let phase = r!(phase.get());
    for (mut image_node, mut tooltip, mut interaction_disabled) in &mut phase_display_query {
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
                Phase::Helm => "[b]Player phase[r]\n\nClick here to end your turn.",
                Phase::Reactor | Phase::Player => {
                    "[b]Reactor phase[r]\n\nDirecting power to the reactor."
                },
                Phase::Enemy => "[b]Enemy phase[r]\n\nSustaining the enemy's barrage!",
            }))
            .with_justify(JustifyText::Center),
        );
        interaction_disabled.0 = !matches!(phase, Phase::Helm);
    }
}

fn sync_phase_display_glow(
    player_deck: Res<PlayerDeck>,
    phase: CurrentRef<Phase>,
    mut phase_display_query: Query<&mut BoxShadow, With<PhaseDisplay>>,
) {
    let shadows = if phase.is_in(&Phase::Helm)
        && (player_deck.hand.is_empty()
            || player_deck
                .reactor
                .iter()
                .all(|slot| matches!(slot.status, ModuleStatus::SlotInactive)))
    {
        vec![ShadowStyle {
            color: Color::srgba_u8(55, 255, 255, 140),
            x_offset: Val::ZERO,
            y_offset: Val::ZERO,
            spread_radius: Val::ZERO,
            blur_radius: Vw(1.0),
        }]
    } else {
        vec![]
    };
    for mut box_shadow in &mut phase_display_query {
        box_shadow.0 = shadows.clone();
    }
}
