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
                Pickable::IGNORE,
                children![storage(hud_assets)],
            ),
        ],
    )
}

fn hand() -> impl Bundle {
    (
        Name::new("Hand"),
        Node::ROW_CENTER.full_size().abs(),
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
                        padding: UiRect::all(Vw(0.61)),
                        ..Node::COLUMN_CENTER.full_height()
                    },
                    Tooltip::fixed(Anchor::TopCenter, item.description()),
                    HandIndex(i),
                    children![(
                        module(&hud_assets, item),
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
pub struct HandIndex(usize);

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
    mut module_query: Query<(&mut Node, &HandIndex)>,
    mut player_deck: ResMut<PlayerDeck>,
) {
    let target = rq!(trigger.get_target());
    let (_, index) = rq!(module_query.get_mut(target));
    player_deck.bypass_change_detection().selected_idx = index.0;
}

fn play_module_on_click(
    trigger: Trigger<Pointer<Click>>,
    module_query: Query<(), With<HandIndex>>,
    mut player_actions: ResMut<ActionState<PlayerActions>>,
) {
    let target = rq!(trigger.get_target());
    rq!(module_query.contains(target));
    player_actions.press(&PlayerActions::PlayModule);
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
