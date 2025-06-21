use crate::animation::shake::Trauma;
use crate::deck::PlayerDeck;
use crate::hud::HudConfig;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(StorageDisplay, StorageLabel)>();
}

pub(super) fn storage_display(hud_config: &HudConfig, game_assets: &GameplayAssets) -> impl Bundle {
    (
        Name::new("StorageDisplay"),
        StorageDisplay,
        ImageNode::from(game_assets.module_face_down.clone()),
        Node {
            width: Vw(6.6666),
            aspect_ratio: Some(1.0),
            padding: UiRect::bottom(Vw(0.2083)).with_left(Vw(0.2083)),
            ..Node::ROW.center()
        },
        hud_config.module_shake,
        Tooltip::fixed(Anchor::BottomLeft, ""),
        children![(
            widget::small_colored_label(ThemeColor::IconText, ""),
            StorageLabel,
        )],
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct StorageDisplay;

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
    mut trauma: Single<&mut Trauma, With<StorageDisplay>>,
) {
    let hud_config = r!(hud_config.get());
    rq!(player_deck.just_used_storage);
    player_deck.just_used_storage = false;
    trauma.0 += hud_config.module_flux_trauma.sample(0.0);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct StorageLabel;

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
