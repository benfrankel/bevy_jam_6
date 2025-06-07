use crate::animation::shake::NodeShake;
use crate::game::GameAssets;
use crate::game::deck::PlayerDeck;
use crate::game::hud::HudConfig;
use crate::game::hud::flux::flux_display;
use crate::game::hud::module::module;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsModuleGrid>();
}

pub fn reactor(game_assets: &GameAssets) -> impl Bundle {
    (
        Name::new("Reactor"),
        ImageNode::from(game_assets.reactor.clone()),
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
    game_assets: Res<GameAssets>,
    hud_config: ConfigRef<HudConfig>,
    player_deck: Res<PlayerDeck>,
    grid_query: Query<Entity, With<IsModuleGrid>>,
) {
    let hud_config = r!(hud_config.get());
    for entity in &grid_query {
        commands
            .entity(entity)
            .despawn_related::<Children>()
            .with_children(|parent| {
                for (i, &slot) in player_deck.reactor.iter().enumerate() {
                    let mut shake = NodeShake::default();
                    if let Some(last_touched) = player_deck.last_touched_idx {
                        if last_touched == i {
                            let factor = hud_config
                                .module_shake_flux_factor
                                .powf(player_deck.flux.max(hud_config.module_shake_flux_min) - 1.0);
                            shake.amplitude = hud_config.module_shake_amplitude;
                            shake.trauma = hud_config.module_shake_trauma * factor;
                            shake.decay = hud_config.module_shake_decay;
                            shake.exponent = hud_config.module_shake_exponent;
                        }
                    }
                    parent.spawn((
                        module(&game_assets, slot, player_deck.heat_capacity, shake),
                        Tooltip::fixed(Anchor::CenterRight, slot.description()),
                    ));
                }
            });
    }
}
