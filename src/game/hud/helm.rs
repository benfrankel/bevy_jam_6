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
use crate::game::module::ModuleAction;
use crate::game::phase::Phase;
use crate::game::phase::helm::HelmActions;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAction;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(IsPhaseDisplay, IsHand, HandIndex, IsStorage, IsStorageLabel)>();
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
            parse_rich("[b]Instruction Manual (I)"),
            open_help_menu,
        ),
    )
}

fn pause_button(game_assets: &GameAssets) -> impl Bundle {
    (
        Name::new("PauseButton"),
        mini_button_base(
            game_assets.pause_button.clone(),
            parse_rich("[b]Pause (P)"),
            open_pause_menu,
        ),
    )
}

fn skip_button(game_assets: &GameAssets) -> impl Bundle {
    (
        Name::new("SkipButton"),
        mini_button_base(
            game_assets.skip_button.clone(),
            parse_rich("[b]End turn (E)"),
            player_end_turn,
        ),
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
        Tooltip::fixed(Anchor::TopCenter, description),
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
                        module(&game_assets, item, player_deck.heat_capacity),
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
        node.top = if idx.0 == player_deck.selected_idx {
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
    let (_, idx) = rq!(module_query.get_mut(target));
    rq!(idx.0 != player_deck.selected_idx);

    player_deck.bypass_change_detection().selected_idx = idx.0;
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
struct IsStorage;

impl Configure for IsStorage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            (
                sync_storage_tooltip
                    .in_set(UpdateSystems::SyncLate)
                    .run_if(resource_changed::<PlayerDeck>.or(any_match_filter::<Added<Self>>)),
                sync_storage_shake
                    .in_set(UpdateSystems::SyncLate)
                    .run_if(resource_changed::<PlayerDeck>),
            ),
        );
    }
}

fn sync_storage_tooltip(
    player_deck: Res<PlayerDeck>,
    mut storage_query: Query<&mut Tooltip, With<IsStorage>>,
) {
    for mut tooltip in &mut storage_query {
        let total = player_deck.storage.len();
        let starts = player_deck
            .storage
            .iter()
            .filter(|x| x.condition == ModuleAction::Nothing || x.effect == ModuleAction::Nothing)
            .count();
        let repairs = player_deck
            .storage
            .iter()
            .filter(|x| x.condition == ModuleAction::Repair || x.effect == ModuleAction::Repair)
            .count();
        let missiles = player_deck
            .storage
            .iter()
            .filter(|x| x.condition == ModuleAction::Missile || x.effect == ModuleAction::Missile)
            .count();
        let lasers = player_deck
            .storage
            .iter()
            .filter(|x| x.condition == ModuleAction::Laser || x.effect == ModuleAction::Laser)
            .count();
        let fireballs = player_deck
            .storage
            .iter()
            .filter(|x| x.condition == ModuleAction::Fireball || x.effect == ModuleAction::Fireball)
            .count();

        tooltip.content = TooltipContent::Primary(RichText::from_sections(parse_rich(format!(
            "[b]Storage[r]\n\n\
            There {} {} reactor module{} remaining to draw:\n\n\
            - {} [b]starter{}[r]\n\
            - {} [b]repair{}[r]\n\
            - {} [b]missile{}[r]\n\
            - {} [b]laser{}[r]\n\
            - {} [b]fireball{}[r]",
            are(total),
            total,
            plural(total),
            starts,
            plural(starts),
            repairs,
            plural(repairs),
            missiles,
            plural(missiles),
            lasers,
            plural(lasers),
            fireballs,
            plural(fireballs),
        ))));
    }
}

fn plural(num: usize) -> &'static str {
    if num == 1 { "" } else { "s" }
}

fn are(num: usize) -> &'static str {
    if num == 1 { "is" } else { "are" }
}

fn sync_storage_shake(
    mut player_deck: ResMut<PlayerDeck>,
    hud_config: ConfigRef<HudConfig>,
    mut shake: Single<&mut NodeShake, With<IsStorage>>,
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
struct IsStorageLabel;

impl Configure for IsStorageLabel {
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
    mut storage_label_query: Query<&mut RichText, With<IsStorageLabel>>,
) {
    for mut text in &mut storage_label_query {
        *text = RichText::from_sections(parse_rich(player_deck.storage.len().to_string()));
    }
}
