pub mod hand;
mod phase_display;
mod storage;

use bevy::ecs::system::IntoObserverSystem;

use crate::animation::offset::NodeOffset;
use crate::hud::HudConfig;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAction;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((hand::plugin, phase_display::plugin, storage::plugin));
}

pub(super) fn helm(hud_config: &HudConfig, game_assets: &GameplayAssets) -> impl Bundle {
    (
        Name::new("Helm"),
        ImageNode::from(game_assets.helm.clone()),
        Node {
            aspect_ratio: Some(356.0 / 58.0),
            ..Node::ROW.full_width()
        },
        children![
            left_helm(),
            hand::hand_display(),
            right_helm(hud_config, game_assets),
        ],
    )
}

fn left_helm() -> impl Bundle {
    (
        Name::new("LeftHelm"),
        Node {
            left: Vw(-0.2083),
            width: Vw(12.0833),
            ..Node::ROW.center().full_height()
        },
        children![phase_display::phase_display()],
    )
}

fn right_helm(hud_config: &HudConfig, game_assets: &GameplayAssets) -> impl Bundle {
    (
        Name::new("RightHelm"),
        Node {
            width: Vw(12.0833),
            padding: UiRect::bottom(Vw(1.25)).with_top(Vw(0.885417)),
            row_gap: Vw(0.052083),
            ..Node::COLUMN.center().full_height()
        },
        children![
            mini_buttons(game_assets),
            storage::storage_display(hud_config, game_assets),
        ],
    )
}

fn mini_buttons(game_assets: &GameplayAssets) -> impl Bundle {
    (
        Name::new("MiniButtons"),
        Node {
            column_gap: Vw(-0.1041666),
            ..Node::ROW
        },
        children![
            info_button(game_assets),
            overlay_button(game_assets),
            pause_button(game_assets),
        ],
    )
}

fn info_button(game_assets: &GameplayAssets) -> impl Bundle {
    mini_button_base(
        game_assets.info_button.clone(),
        parse_rich("[b]Instruction Manual (I)"),
        open_help_menu,
    )
}

fn overlay_button(game_assets: &GameplayAssets) -> impl Bundle {
    mini_button_base(
        game_assets.skip_button.clone(),
        parse_rich("[b]Reactor Overlay (O)"),
        toggle_reactor_overlay,
    )
}

fn pause_button(game_assets: &GameplayAssets) -> impl Bundle {
    mini_button_base(
        game_assets.pause_button.clone(),
        parse_rich("[b]Pause (P)"),
        open_pause_menu,
    )
}

fn open_help_menu(
    trigger: Trigger<Pointer<Click>>,
    mut gameplay_action: ResMut<ActionState<GameplayAction>>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    gameplay_action.press(&GameplayAction::ToggleHelp);
}

fn toggle_reactor_overlay(
    trigger: Trigger<Pointer<Click>>,
    mut _gameplay_action: ResMut<ActionState<GameplayAction>>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    // TODO: gameplay_action.press(&GameplayAction::ToggleOverlay);
}

fn open_pause_menu(
    trigger: Trigger<Pointer<Click>>,
    mut gameplay_action: ResMut<ActionState<GameplayAction>>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    gameplay_action.press(&GameplayAction::Pause);
}

fn mini_button_base<E, B, M, I>(
    image: Handle<Image>,
    description: impl Into<TooltipContent>,
    action: I,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    (
        Name::new("MiniButtonInteractionRegion"),
        Button,
        Node {
            padding: UiRect::all(Vw(0.36458333)),
            ..default()
        },
        InteractionGlassSfx,
        Previous::<Interaction>::default(),
        Tooltip::fixed(Anchor::TopCenter, description),
        Patch(|entity| {
            entity.observe(action);
        }),
        children![(
            Name::new("MiniButton"),
            ImageNode::from(image),
            Node {
                width: Vw(2.5),
                aspect_ratio: Some(1.0),
                ..default()
            },
            NodeOffset::default(),
            ParentInteractionTheme {
                hovered: NodeOffset::new(Val::ZERO, Vw(-0.2083)),
                pressed: NodeOffset::new(Val::ZERO, Vw(0.2083)),
                ..default()
            },
            Pickable::IGNORE,
        )],
    )
}
