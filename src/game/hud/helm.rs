use bevy::ecs::system::IntoObserverSystem;

use crate::animation::offset::NodeOffset;
use crate::animation::shake::NodeShake;
use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::game::GameAssets;
use crate::game::deck::PlayerDeck;
use crate::game::hud::HudConfig;
use crate::game::hud::module::module;
use crate::game::level::Level;
use crate::game::phase::Phase;
use crate::game::phase::helm::HelmActions;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAction;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(IsPhaseDisplay, IsHand, HandIndex, IsStorage, IsStorageLabel)>();
    app.add_systems(
        Update,
        apply_shake_storage_on_draw.run_if(resource_changed::<PlayerDeck>),
    );
}

pub fn helm(game_assets: &GameAssets) -> impl Bundle {
    (
        Name::new("Helm"),
        ImageNode::from(game_assets.helm.clone()),
        Node {
            aspect_ratio: Some(356.0 / 58.0),
            ..Node::ROW.full_width()
        },
        children![left_helm(), hand(), right_helm(game_assets)],
    )
}

fn left_helm() -> impl Bundle {
    (
        Name::new("LeftHelm"),
        Node {
            left: Vw(-0.2083),
            width: Vw(12.0833),
            ..Node::ROW_CENTER.full_height()
        },
        children![phase_display()],
    )
}

fn phase_display() -> impl Bundle {
    (
        Name::new("PhaseDisplay"),
        IsPhaseDisplay,
        ImageNode::default(),
        Node {
            width: Vw(9.1666),
            aspect_ratio: Some(1.0),
            ..Node::DEFAULT
        },
        Tooltip::fixed(Anchor::TopCenter, ""),
    )
}

fn hand() -> impl Bundle {
    (Name::new("Hand"), IsHand, Node::ROW_CENTER.grow())
}

fn right_helm(game_assets: &GameAssets) -> impl Bundle {
    (
        Name::new("RightHelm"),
        Node {
            width: Vw(12.0833),
            padding: UiRect::vertical(Vw(1.25)),
            row_gap: Vw(0.41666),
            ..Node::COLUMN_CENTER.full_height()
        },
        children![mini_buttons(game_assets), storage(game_assets)],
    )
}

fn storage(game_assets: &GameAssets) -> impl Bundle {
    (
        Name::new("Storage"),
        IsStorage,
        ImageNode::from(game_assets.module_face_down.clone()),
        Node {
            width: Vw(6.6666),
            aspect_ratio: Some(1.0),
            padding: UiRect::bottom(Vw(0.2083)).with_left(Vw(0.2083)),
            ..Node::ROW_CENTER
        },
        Tooltip::fixed(Anchor::TopCenter, ""),
        NodeShake::default(),
        children![(
            widget::small_colored_label(ThemeColor::IconText, ""),
            IsStorageLabel,
        )],
    )
}

fn mini_buttons(game_assets: &GameAssets) -> impl Bundle {
    (
        Name::new("MiniButtons"),
        Node {
            column_gap: Vw(0.625),
            ..Node::ROW
        },
        children![
            info_button(game_assets),
            pause_button(game_assets),
            skip_button(game_assets),
        ],
    )
}

fn info_button(game_assets: &GameAssets) -> impl Bundle {
    (
        Name::new("InfoButton"),
        mini_button_base(
            game_assets.info_button.clone(),
            "[b]Instruction Manual (I)[r]\n\nRead the ship's instruction manual.",
            toggle_tooltips,
        ),
    )
}

fn pause_button(game_assets: &GameAssets) -> impl Bundle {
    (
        Name::new("PauseButton"),
        mini_button_base(
            game_assets.pause_button.clone(),
            "[b]Pause (P)[r]\n\nOpen the pause menu.",
            open_pause_menu,
        ),
    )
}

fn skip_button(game_assets: &GameAssets) -> impl Bundle {
    (
        Name::new("SkipButton"),
        mini_button_base(
            game_assets.skip_button.clone(),
            "[b]End turn (E)[r]\n\nEnd your turn without playing a module from your hand.",
            player_end_turn,
        ),
    )
}

fn toggle_tooltips(
    trigger: Trigger<Pointer<Click>>,
    mut gameplay_action: ResMut<ActionState<GameplayAction>>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    gameplay_action.press(&GameplayAction::ToggleHelp);
}

fn open_pause_menu(
    trigger: Trigger<Pointer<Click>>,
    mut gameplay_action: ResMut<ActionState<GameplayAction>>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    gameplay_action.press(&GameplayAction::Pause);
}

fn player_end_turn(
    trigger: Trigger<Pointer<Click>>,
    mut player_actions: ResMut<ActionState<HelmActions>>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    player_actions.press(&HelmActions::EndTurn);
}

fn mini_button_base<E, B, M, I>(image: Handle<Image>, description: &str, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    (
        Button,
        ImageNode::from(image),
        Node {
            width: Vw(2.5),
            aspect_ratio: Some(1.0),
            ..default()
        },
        NodeOffset::default(),
        InteractionTheme {
            hovered: NodeOffset::new(Val::ZERO, Vw(-0.2083)),
            pressed: NodeOffset::new(Val::ZERO, Vw(0.2083)),
            ..default()
        },
        Tooltip::fixed(Anchor::TopCenter, parse_rich(description)),
        Patch(|entity| {
            entity.observe(action);
        }),
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsPhaseDisplay;

impl Configure for IsPhaseDisplay {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(StateFlush, Phase::ANY.on_enter(sync_phase_display));
    }
}

fn sync_phase_display(
    phase: NextRef<Phase>,
    game_assets: Res<GameAssets>,
    mut phase_display_query: Query<(&mut ImageNode, &mut Tooltip), With<IsPhaseDisplay>>,
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
        tooltip.content =
            TooltipContent::Primary(RichText::from_sections(parse_rich(match phase {
                Phase::Setup => "[b]Setup phase[r]\n\nPreparing the ship.",
                Phase::Helm => "[b]Player phase[r]\n\nAwaiting your command.",
                Phase::Reactor | Phase::Player => {
                    "[b]Reactor phase[r]\n\nDirecting power to the reactor."
                },
                Phase::Enemy => "[b]Enemy phase[r]\n\nSustaining the enemy's barrage.",
            })));
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsHand;

impl Configure for IsHand {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            sync_hand
                .in_set(UpdateSystems::SyncLate)
                .run_if(resource_changed::<PlayerDeck>.or(any_match_filter::<Added<Self>>)),
        );
    }
}

fn sync_hand(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    player_deck: Res<PlayerDeck>,
    hand: Single<Entity, With<IsHand>>,
) {
    let selected_idx = player_deck.selected_idx;
    commands
        .entity(*hand)
        .despawn_related::<Children>()
        .with_children(|parent| {
            for (i, &item) in player_deck.hand.iter().enumerate() {
                parent.spawn((
                    Name::new("ModuleInteractionRegion"),
                    Node {
                        padding: UiRect::all(Vw(0.4167)),
                        ..Node::COLUMN_CENTER.full_height()
                    },
                    Tooltip::fixed(Anchor::BottomCenter, parse_rich(item.short_description())),
                    HandIndex(i),
                    children![(
                        module(
                            &game_assets,
                            item,
                            player_deck.heat_capacity,
                            NodeShake::default()
                        ),
                        Patch(move |entity| {
                            if i == selected_idx {
                                r!(entity.get_mut::<Node>()).top = Vw(-2.0);
                            }
                        }),
                    )],
                ));
            }
        });
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HandIndex(pub usize);

impl Configure for HandIndex {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_offset_to_selected_module.in_set(UpdateSystems::Update),
        );
        app.add_observer(select_module_on_hover);
        app.add_observer(play_module_on_click);
    }
}

fn apply_offset_to_selected_module(
    player_deck: Res<PlayerDeck>,
    mut module_query: Query<(&mut Node, &ChildOf)>,
    container_query: Query<&HandIndex>,
) {
    for (mut node, child_of) in &mut module_query {
        let index = cq!(container_query.get(child_of.parent()));
        node.top = if index.0 == player_deck.selected_idx {
            Vw(-2.0)
        } else {
            Val::ZERO
        }
    }
}

fn select_module_on_hover(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    game_assets: Res<GameAssets>,
    mut module_query: Query<(&mut Node, &HandIndex)>,
    mut player_deck: ResMut<PlayerDeck>,
) {
    let target = rq!(trigger.get_target());
    let (_, index) = rq!(module_query.get_mut(target));

    player_deck.bypass_change_detection().selected_idx = index.0;
    commands.spawn((
        sfx_audio(&audio_settings, game_assets.module_hover_sfx.clone(), 1.0),
        DespawnOnExitState::<Level>::default(),
    ));
}

fn play_module_on_click(
    trigger: Trigger<Pointer<Click>>,
    module_query: Query<(), With<HandIndex>>,
    mut player_actions: ResMut<ActionState<HelmActions>>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    let target = rq!(trigger.get_target());
    rq!(module_query.contains(target));
    player_actions.press(&HelmActions::PlayModule);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsStorage;

impl Configure for IsStorage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            sync_storage_tooltip
                .in_set(UpdateSystems::SyncLate)
                .run_if(resource_changed::<PlayerDeck>.or(any_match_filter::<Added<Self>>)),
        );
    }
}

fn sync_storage_tooltip(
    player_deck: Res<PlayerDeck>,
    mut storage_query: Query<&mut Tooltip, With<IsStorage>>,
) {
    for mut tooltip in &mut storage_query {
        tooltip.content = TooltipContent::Primary(RichText::from_sections(parse_rich(format!(
            "[b]Storage[r]\n\n{} reactor module{} remaining.",
            player_deck.storage.len(),
            if player_deck.storage.len() == 1 {
                ""
            } else {
                "s"
            },
        ))));
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsStorageLabel;

impl Configure for IsStorageLabel {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            sync_storage_label
                .in_set(UpdateSystems::SyncLate)
                .run_if(resource_changed::<PlayerDeck>.or(any_match_filter::<Added<Self>>)),
        );
    }
}

fn sync_storage_label(
    player_deck: Res<PlayerDeck>,
    mut storage_label_query: Query<&mut RichText, With<IsStorageLabel>>,
) {
    for mut text in &mut storage_label_query {
        *text = RichText::from_sections(parse_rich(player_deck.storage.len().to_string()));
    }
}

fn apply_shake_storage_on_draw(
    mut player_deck: ResMut<PlayerDeck>,
    hud_config: ConfigRef<HudConfig>,
    mut shake: Single<&mut NodeShake, With<IsStorage>>,
) {
    let hud_config = r!(hud_config.get());
    if player_deck.just_drew {
        player_deck.just_drew = false;
        shake.trauma = hud_config.module_shake_trauma;
        shake.decay = hud_config.module_shake_decay;
    };
}
