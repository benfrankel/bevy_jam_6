use crate::game::deck::PlayerDeck;
use crate::game::hud::HudAssets;
use crate::game::hud::module::module;
use crate::game::phase::player::PlayerActions;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(IsHand, HandIndex, IsStorage, IsStorageLabel)>();
}

pub fn helm(hud_assets: &HudAssets) -> impl Bundle {
    (
        Name::new("Helm"),
        ImageNode::from(hud_assets.helm.clone()),
        Node {
            aspect_ratio: Some(356.0 / 58.0),
            ..Node::ROW.full_width()
        },
        children![
            hand(),
            (
                Name::new("Row"),
                Node {
                    padding: UiRect::all(Vw(1.69)),
                    ..Node::ROW_MID.reverse().full_width()
                },
                children![storage(hud_assets)],
            ),
        ],
    )
}

fn hand() -> impl Bundle {
    (
        Name::new("Hand"),
        Node {
            column_gap: Vw(1.22),
            ..Node::ROW_CENTER.full_size().abs()
        },
        IsHand,
    )
}

fn storage(hud_assets: &HudAssets) -> impl Bundle {
    (
        Name::new("Storage"),
        ImageNode::from(hud_assets.module_face_down.clone()),
        Node {
            width: Vw(6.66),
            aspect_ratio: Some(1.0),
            ..Node::ROW_CENTER
        },
        Tooltip::fixed(Anchor::TopCenter, ""),
        IsStorage,
        children![(
            widget::small_colored_label("", ThemeColor::IconText),
            IsStorageLabel,
        )],
    )
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
    hud_assets: Res<HudAssets>,
    player_deck: Res<PlayerDeck>,
    hand_query: Query<Entity, With<IsHand>>,
) {
    for entity in &hand_query {
        commands
            .entity(entity)
            .despawn_related::<Children>()
            .with_children(|parent| {
                for (i, &card) in player_deck.hand.iter().enumerate() {
                    if i == player_deck.selected_idx {
                        parent.spawn((
                            module(&hud_assets, card, Anchor::TopCenter),
                            HandIndex(i),
                            Patch(|entity| r!(entity.get_mut::<Node>()).top = Vw(-2.0)),
                        ));
                    } else {
                        parent.spawn((module(&hud_assets, card, Anchor::TopCenter), HandIndex(i)));
                    }
                }
            });
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct HandIndex(usize);

impl Configure for HandIndex {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            (
                select_module_on_hover.in_set(UpdateSystems::Update),
                play_module_on_click.in_set(UpdateSystems::Update),
            ),
        );
    }
}

fn select_module_on_hover(
    hand_index_query: Query<(&Interaction, &HandIndex)>,
    mut player_deck: ResMut<PlayerDeck>,
) {
    for (&interaction, index) in &hand_index_query {
        if interaction == Interaction::Hovered && player_deck.selected_idx != index.0 {
            player_deck.selected_idx = index.0;
            break;
        }
    }
}

fn play_module_on_click(
    interaction_query: Query<&Interaction, With<HandIndex>>,
    mut player_actions: ResMut<ActionState<PlayerActions>>,
) {
    if interaction_query.iter().any(|&x| x == Interaction::Pressed) {
        player_actions.press(&PlayerActions::PlayModule);
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
