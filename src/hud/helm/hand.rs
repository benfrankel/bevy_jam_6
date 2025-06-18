use crate::animation::shake::NodeShake;
use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::deck::PlayerDeck;
use crate::hud::module::module;
use crate::level::Level;
use crate::module::ModuleConfig;
use crate::phase::helm::HelmActions;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(HandDisplay, HandIndex)>();
}

pub(super) fn hand_display() -> impl Bundle {
    (
        Name::new("HandDisplay"),
        HandDisplay,
        Node {
            column_gap: Px(-1.0),
            ..Node::ROW.center().grow()
        },
    )
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
                        ..Node::COLUMN.center().full_height()
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
