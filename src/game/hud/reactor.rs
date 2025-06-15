use crate::animation::shake::NodeShake;
use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::game::GameAssets;
use crate::game::deck::PlayerDeck;
use crate::game::hud::HudConfig;
use crate::game::hud::flux::flux_display;
use crate::game::hud::module::module;
use crate::game::level::Level;
use crate::game::module::ModuleConfig;
use crate::game::module::ModuleStatus;
use crate::game::phase::Phase;
use crate::game::projectile::ProjectileConfig;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ReactorGrid, ReactorIndex)>();
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
        children![flux_display(), reactor_grid()],
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
struct ReactorGrid;

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
    game_assets: Res<GameAssets>,
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
                        module(&game_assets, module_config, slot, player_deck.heat_capacity),
                        shake,
                        ReactorIndex(i),
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
    game_assets: Res<GameAssets>,
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
    game_assets: Res<GameAssets>,
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
