use crate::game::deck::PlayerDeck;
use crate::game::hud::HudAssets;
use crate::game::hud::flux::flux_display;
use crate::game::hud::module::module;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsModuleGrid>();
}

pub fn reactor(hud_assets: &HudAssets) -> impl Bundle {
    (
        Name::new("Reactor"),
        ImageNode::from(hud_assets.reactor.clone()),
        Node {
            aspect_ratio: Some(124.0 / 270.0),
            padding: UiRect::all(Vw(1.69)),
            row_gap: Vw(1.69),
            ..Node::COLUMN_MID.full_height()
        },
        children![flux_display(), module_grid()],
    )
}

fn module_grid() -> impl Bundle {
    (
        Name::new("ModuleGrid"),
        Node {
            display: Display::Grid,
            row_gap: Vw(1.25),
            column_gap: Vw(1.25),
            grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
            ..Node::default().full_width()
        },
        IsModuleGrid,
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsModuleGrid;

impl Configure for IsModuleGrid {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            sync_module_grid
                .in_set(UpdateSystems::SyncLate)
                .run_if(resource_changed::<PlayerDeck>.or(any_match_filter::<Added<Self>>)),
        );
    }
}

fn sync_module_grid(
    mut commands: Commands,
    hud_assets: Res<HudAssets>,
    player_deck: Res<PlayerDeck>,
    grid_query: Query<Entity, With<IsModuleGrid>>,
) {
    for entity in &grid_query {
        commands
            .entity(entity)
            .despawn_related::<Children>()
            .with_children(|parent| {
                for &slot in &player_deck.reactor {
                    parent.spawn((
                        module(&hud_assets, slot, player_deck.heat_capacity),
                        Tooltip::fixed(Anchor::CenterRight, slot.description()),
                    ));
                }
            });
    }
}
