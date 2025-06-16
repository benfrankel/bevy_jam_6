use bevy::ecs::system::IntoObserverSystem;

use crate::animation::offset::NodeOffset;
use crate::animation::shake::NodeShake;
use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::deck::PlayerDeck;
use crate::hud::HudConfig;
use crate::hud::module::module;
use crate::level::Level;
use crate::module::ModuleConfig;
use crate::phase::Phase;
use crate::phase::helm::HelmActions;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAction;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        PhaseDisplay,
        HandDisplay,
        HandIndex,
        StorageDisplay,
        StorageLabel,
    )>();
}

pub fn helm(game_assets: &GameplayAssets) -> impl Bundle {
    (
        Name::new("Helm"),
        ImageNode::from(game_assets.helm.clone()),
        Node {
            aspect_ratio: Some(356.0 / 58.0),
            ..Node::ROW.full_width()
        },
        children![left_helm(), hand_display(), right_helm(game_assets)],
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
        PhaseDisplay,
        ImageNode::default(),
        Node {
            width: Vw(9.1666),
            aspect_ratio: Some(1.0),
            ..Node::DEFAULT
        },
        Tooltip::fixed(Anchor::TopCenter, ""),
    )
}

fn hand_display() -> impl Bundle {
    (
        Name::new("HandDisplay"),
        HandDisplay,
        Node {
            column_gap: Px(-1.0),
            ..Node::ROW_CENTER.grow()
        },
    )
}

fn right_helm(game_assets: &GameplayAssets) -> impl Bundle {
    (
        Name::new("RightHelm"),
        Node {
            width: Vw(12.0833),
            padding: UiRect::bottom(Vw(1.25)).with_top(Vw(0.885417)),
            row_gap: Vw(0.052083),
            ..Node::COLUMN_CENTER.full_height()
        },
        children![mini_buttons(game_assets), storage_display(game_assets)],
    )
}

fn storage_display(game_assets: &GameplayAssets) -> impl Bundle {
    (
        Name::new("StorageDisplay"),
        StorageDisplay,
        ImageNode::from(game_assets.module_face_down.clone()),
        Node {
            width: Vw(6.6666),
            aspect_ratio: Some(1.0),
            padding: UiRect::bottom(Vw(0.2083)).with_left(Vw(0.2083)),
            ..Node::ROW_CENTER
        },
        Tooltip::fixed(Anchor::BottomLeft, ""),
        NodeShake::default(),
        children![(
            widget::small_colored_label(ThemeColor::IconText, ""),
            StorageLabel,
        )],
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
            pause_button(game_assets),
            skip_button(game_assets),
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

fn pause_button(game_assets: &GameplayAssets) -> impl Bundle {
    mini_button_base(
        game_assets.pause_button.clone(),
        parse_rich("[b]Pause (P)"),
        open_pause_menu,
    )
}

fn skip_button(game_assets: &GameplayAssets) -> impl Bundle {
    mini_button_base(
        game_assets.skip_button.clone(),
        parse_rich("[b]End turn (E)"),
        player_end_turn,
    )
}

fn open_help_menu(
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
        Tooltip::fixed(Anchor::TopCenter, description),
        Previous::<Interaction>::default(),
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
                Phase::Setup => "[b]Storage phase[r]\n\nPulling reactor modules from storage.",
                Phase::Helm => {
                    "[b]Player phase[r]\n\nLeft click a module to play it, right click to discard."
                },
                Phase::Reactor | Phase::Player => {
                    "[b]Reactor phase[r]\n\nDirecting power to the reactor."
                },
                Phase::Enemy => "[b]Enemy phase[r]\n\nSustaining the enemy's barrage.",
            }))
            .with_justify(JustifyText::Center),
        );
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct HandDisplay;

impl Configure for HandDisplay {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            sync_hand_display
                .in_set(UpdateSystems::SyncLate)
                .run_if(resource_changed::<PlayerDeck>.or(any_match_filter::<Added<Self>>)),
        );
    }
}

fn sync_hand_display(
    mut commands: Commands,
    module_config: ConfigRef<ModuleConfig>,
    game_assets: Res<GameplayAssets>,
    player_deck: Res<PlayerDeck>,
    hand: Single<Entity, With<HandDisplay>>,
) {
    let module_config = r!(module_config.get());
    let selected_idx = player_deck.hand_idx;
    commands
        .entity(*hand)
        .despawn_related::<Children>()
        .with_children(|parent| {
            for (i, item) in player_deck.hand.iter().enumerate() {
                parent.spawn((
                    Name::new("ModuleInteractionRegion"),
                    Node {
                        padding: UiRect::all(Vw(0.4167)),
                        ..Node::COLUMN_CENTER.full_height()
                    },
                    Tooltip::fixed(
                        Anchor::BottomCenter,
                        parse_rich(item.short_description(module_config)),
                    ),
                    HandIndex(i),
                    children![(
                        module(&game_assets, module_config, item, player_deck.heat_capacity),
                        Pickable::IGNORE,
                        NodeShake::default(),
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
        app.add_observer(play_or_discard_module_on_click);
    }
}

fn apply_offset_to_selected_module(
    player_deck: Res<PlayerDeck>,
    mut module_query: Query<(&mut Node, &ChildOf)>,
    container_query: Query<&HandIndex>,
) {
    for (mut node, child_of) in &mut module_query {
        let idx = cq!(container_query.get(child_of.parent()));
        node.top = if idx.0 == player_deck.hand_idx {
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
    game_assets: Res<GameplayAssets>,
    mut module_query: Query<(&mut Node, &HandIndex)>,
    mut player_deck: ResMut<PlayerDeck>,
) {
    let target = rq!(trigger.get_target());
    let (_, idx) = rq!(module_query.get_mut(target));
    rq!(idx.0 != player_deck.hand_idx);

    player_deck.bypass_change_detection().hand_idx = idx.0;
    commands.spawn((
        sfx_audio(&audio_settings, game_assets.module_hover_sfx.clone(), 1.0),
        DespawnOnExitState::<Level>::default(),
    ));
}

fn play_or_discard_module_on_click(
    trigger: Trigger<Pointer<Click>>,
    module_query: Query<(), With<HandIndex>>,
    mut player_actions: ResMut<ActionState<HelmActions>>,
) {
    let target = rq!(trigger.get_target());
    rq!(module_query.contains(target));

    match trigger.event.button {
        PointerButton::Primary => player_actions.press(&HelmActions::PlayModule),
        PointerButton::Secondary => player_actions.press(&HelmActions::DiscardModule),
        PointerButton::Middle => {},
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct StorageDisplay;

impl Configure for StorageDisplay {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            (
                sync_storage_display_tooltip
                    .in_set(UpdateSystems::SyncLate)
                    .run_if(resource_changed::<PlayerDeck>.or(any_match_filter::<Added<Self>>)),
                sync_storage_display_shake
                    .in_set(UpdateSystems::SyncLate)
                    .run_if(resource_changed::<PlayerDeck>),
            ),
        );
    }
}

fn sync_storage_display_tooltip(
    hud_config: ConfigRef<HudConfig>,
    player_deck: Res<PlayerDeck>,
    mut storage_query: Query<&mut Tooltip, With<StorageDisplay>>,
) {
    let hud_config = r!(hud_config.get());

    for mut tooltip in &mut storage_query {
        let total = player_deck.storage.len();
        let mut counts = vec![];
        for action in &hud_config.storage_summary_actions {
            let count = player_deck
                .storage
                .iter()
                .filter(|x| &x.condition == action || &x.effect == action)
                .count();
            counts.push(format!(
                "- {} [b]{}{}[r]",
                count,
                if action.is_empty() { "starter" } else { action },
                plural(count)
            ));
        }

        tooltip.content = TooltipContent::Primary(RichText::from_sections(parse_rich(format!(
            "[b]Storage[r]\n\nThere {} {} reactor module{} remaining to draw:\n\n{}",
            are(total),
            total,
            plural(total),
            counts.join("\n"),
        ))));
    }
}

fn plural(num: usize) -> &'static str {
    if num == 1 { "" } else { "s" }
}

fn are(num: usize) -> &'static str {
    if num == 1 { "is" } else { "are" }
}

fn sync_storage_display_shake(
    mut player_deck: ResMut<PlayerDeck>,
    hud_config: ConfigRef<HudConfig>,
    mut shake: Single<&mut NodeShake, With<StorageDisplay>>,
) {
    let hud_config = r!(hud_config.get());
    rq!(player_deck.just_used_storage);
    player_deck.just_used_storage = false;

    let factor = hud_config
        .module_shake_flux_factor
        .powf(hud_config.module_shake_flux_min - 1.0);
    shake.amplitude = hud_config.module_shake_amplitude;
    shake.trauma = hud_config.module_shake_trauma * factor;
    shake.decay = hud_config.module_shake_decay;
    shake.exponent = hud_config.module_shake_exponent;
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct StorageLabel;

impl Configure for StorageLabel {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            (sync_storage_label
                .in_set(UpdateSystems::SyncLate)
                .run_if(resource_changed::<PlayerDeck>.or(any_match_filter::<Added<Self>>)),),
        );
    }
}

fn sync_storage_label(
    player_deck: Res<PlayerDeck>,
    mut storage_label_query: Query<&mut RichText, With<StorageLabel>>,
) {
    for mut text in &mut storage_label_query {
        *text = RichText::from_sections(parse_rich(player_deck.storage.len().to_string()));
    }
}
