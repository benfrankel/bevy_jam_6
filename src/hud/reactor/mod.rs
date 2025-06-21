pub mod flux_display;

use crate::animation::shake::Trauma;
use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::deck::PlayerDeck;
use crate::hud::HudConfig;
use crate::hud::module::module;
use crate::level::Level;
use crate::module::ModuleConfig;
use crate::module::ModuleStatus;
use crate::phase::Phase;
use crate::prelude::*;
use crate::projectile::ProjectileConfig;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(flux_display::plugin);

    app.configure::<(ReactorGrid, ReactorIndex)>();
}

pub(super) fn reactor(hud_config: &HudConfig, game_assets: &GameplayAssets) -> impl Bundle {
    (
        Name::new("Reactor"),
        ImageNode::from(game_assets.reactor.clone()),
        Node {
            aspect_ratio: Some(124.0 / 270.0),
            padding: UiRect::all(Vw(1.69)),
            row_gap: Vw(1.69),
            ..Node::COLUMN.top_center().full_height()
        },
        children![flux_display::flux_display(hud_config), reactor_grid()],
    )
}

fn reactor_grid() -> impl Bundle {
    (
        Name::new("ReactorGrid"),
        Node {
            display: Display::Grid,
            row_gap: Vw(1.25),
            column_gap: Vw(1.25),
            grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
            ..Node::default().full_width()
        },
        ReactorGrid,
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ReactorGrid;

impl Configure for ReactorGrid {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            sync_reactor_grid
                .in_set(UpdateSystems::SyncLate)
                .run_if(resource_changed::<PlayerDeck>.or(any_match_filter::<Added<Self>>)),
        );
    }
}

fn sync_reactor_grid(
    mut commands: Commands,
    game_assets: Res<GameplayAssets>,
    hud_config: ConfigRef<HudConfig>,
    module_config: ConfigRef<ModuleConfig>,
    projectile_config: ConfigRef<ProjectileConfig>,
    player_deck: Res<PlayerDeck>,
    grid_query: Query<Entity, With<ReactorGrid>>,
) {
    let hud_config = r!(hud_config.get());
    let module_config = r!(module_config.get());
    let projectile_config = r!(projectile_config.get());
    for entity in &grid_query {
        commands
            .entity(entity)
            .despawn_related::<Children>()
            .with_children(|parent| {
                for (i, slot) in player_deck.reactor.iter().enumerate() {
                    let mut trauma = 0.0;
                    if let Some(last_touched) = player_deck.last_touched_idx {
                        if last_touched == i {
                            trauma += hud_config.module_flux_trauma.sample(player_deck.flux);
                        }
                    }

                    parent.spawn((
                        ReactorIndex(i),
                        module(&game_assets, module_config, slot, player_deck.heat_capacity),
                        hud_config.module_shake,
                        Trauma(trauma),
                        Tooltip::fixed(
                            Anchor::CenterRight,
                            parse_rich(slot.description(
                                module_config,
                                projectile_config,
                                player_deck.heat_capacity,
                            )),
                        ),
                        Patch(|entity| {
                            entity.observe(play_hover_sfx_on_hover);
                        }),
                    ));
                }
            });
    }
}

fn play_hover_sfx_on_hover(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    reactor_module_query: Query<Ref<ReactorIndex>>,
    player_deck: Res<PlayerDeck>,
    audio_settings: Res<AudioSettings>,
    game_assets: Res<GameplayAssets>,
) {
    let target = rq!(trigger.get_target());
    let idx = rq!(reactor_module_query.get(target));
    rq!(!matches!(
        player_deck.reactor[idx.0].status,
        ModuleStatus::SlotEmpty,
    ));
    rq!(!idx.is_added());

    commands.spawn((
        sfx_audio(&audio_settings, game_assets.module_hover_sfx.clone(), 1.0),
        DespawnOnExitState::<Level>::default(),
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ReactorIndex(pub usize);

impl Configure for ReactorIndex {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(discard_module_on_right_click);
    }
}

fn discard_module_on_right_click(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    phase: NextRef<Phase>,
    reactor_module_query: Query<&ReactorIndex>,
    mut player_deck: ResMut<PlayerDeck>,
    audio_settings: Res<AudioSettings>,
    game_assets: Res<GameplayAssets>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Secondary));
    rq!(matches!(phase.get(), Some(Phase::Helm)));
    let target = rq!(trigger.get_target());
    let idx = rq!(reactor_module_query.get(target));
    rq!(!matches!(
        player_deck.reactor[idx.0].status,
        ModuleStatus::SlotEmpty,
    ));

    player_deck.discard_module(idx.0);
    commands.spawn((
        sfx_audio(&audio_settings, game_assets.module_insert_sfx.clone(), 1.0),
        DespawnOnExitState::<Level>::default(),
    ));
}
