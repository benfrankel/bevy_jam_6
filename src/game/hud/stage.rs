use crate::game::deck::Deck;
use crate::game::deck::Module;
use crate::game::deck::ModuleStatus;
use crate::game::hud::HudAssets;
use crate::game::hud::module::module;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(IsHand, IsStorage, IsStorageLabel)>();
}

pub fn stage(hud_assets: &HudAssets) -> impl Bundle {
    (
        Name::new("Stage"),
        ImageNode::from(hud_assets.stage.clone()),
        Node {
            aspect_ratio: Some(356.0 / 58.0),
            ..Node::ROW.full_width()
        },
        children![
            hand(hud_assets),
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

fn hand(hud_assets: &HudAssets) -> impl Bundle {
    let face_up = Module {
        status: ModuleStatus::FaceUp,
        ..default()
    };

    (
        Name::new("Hand"),
        Node {
            column_gap: Vw(1.22),
            ..Node::ROW_CENTER.full_size().abs()
        },
        IsHand,
        children![
            module(hud_assets, face_up, Anchor::TopCenter),
            module(hud_assets, face_up, Anchor::TopCenter),
            module(hud_assets, face_up, Anchor::TopCenter),
            module(hud_assets, face_up, Anchor::TopCenter),
            module(hud_assets, face_up, Anchor::TopCenter),
        ],
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
        Tooltip::fixed(Anchor::TopCenter, parse_rich("[b]Storage")),
        IsStorage,
        children![(
            widget::small_colored_label("15", ThemeColor::IconText),
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
                .run_if(resource_changed::<Deck>.or(any_match_filter::<Added<Self>>)),
        );
    }
}

fn sync_hand(
    mut commands: Commands,
    hud_assets: Res<HudAssets>,
    deck: Res<Deck>,
    hand_query: Query<Entity, With<IsHand>>,
) {
    for entity in &hand_query {
        commands
            .entity(entity)
            .despawn_related::<Children>()
            .with_children(|parent| {
                for &card in &deck.hand {
                    parent.spawn(module(&hud_assets, card, Anchor::TopCenter));
                }
            });
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
                .run_if(resource_changed::<Deck>.or(any_match_filter::<Added<Self>>)),
        );
    }
}

fn sync_storage_tooltip(deck: Res<Deck>, mut storage_query: Query<&mut Tooltip, With<IsStorage>>) {
    for mut tooltip in &mut storage_query {
        tooltip.content = TooltipContent::Primary(RichText::from_sections(parse_rich(format!(
            "[b]Storage[r]\n\n{} reactor module{} remaining.",
            deck.storage.len(),
            if deck.storage.len() == 1 { "" } else { "s" },
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
                .run_if(resource_changed::<Deck>.or(any_match_filter::<Added<Self>>)),
        );
    }
}

fn sync_storage_label(
    deck: Res<Deck>,
    mut storage_label_query: Query<&mut RichText, With<IsStorageLabel>>,
) {
    for mut text in &mut storage_label_query {
        *text = RichText::from_sections(parse_rich(deck.storage.len().to_string()));
    }
}
