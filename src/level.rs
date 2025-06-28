use crate::animation::shake::Shake;
use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::core::camera::CameraRoot;
use crate::core::physics::GameLayer;
use crate::deck::DeckConfig;
use crate::deck::EnemyDeck;
use crate::deck::PlayerDeck;
use crate::hud;
use crate::hud::HudConfig;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;
use crate::ship::ShipConfig;
use crate::ship::enemy_ship;
use crate::ship::player_ship;
use crate::theme::toast::Toaster;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<LevelConfig>, Level)>();
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct LevelConfig {
    pub levels: Vec<LevelSetup>,
}

impl Config for LevelConfig {
    const FILE: &'static str = "level.ron";
}

#[derive(Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct LevelSetup {
    pub name: String,
    pub enemy_deck: EnemyDeck,
}

#[derive(State, Reflect, Copy, Clone, Default, Eq, PartialEq, Debug)]
#[state(log_flush, react, before(Menu))]
#[reflect(Resource)]
pub struct Level(pub usize);

impl Configure for Level {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(
            StateFlush,
            Level::ANY.on_edge(
                (reset_player_deck, reset_camera),
                (
                    (set_up_decks, spawn_level).chain(),
                    (Menu::release, Menu::clear).chain(),
                ),
            ),
        );
    }
}

fn reset_camera(camera_root: Res<CameraRoot>, mut camera_query: Query<&mut Shake>) {
    let mut shake = r!(camera_query.get_mut(camera_root.primary));
    *shake = default();
}

fn reset_player_deck(mut player_deck: ResMut<PlayerDeck>) {
    player_deck.reset();
}

fn set_up_decks(
    level: NextRef<Level>,
    level_config: ConfigRef<LevelConfig>,
    deck_config: ConfigRef<DeckConfig>,
    mut player_deck: ResMut<PlayerDeck>,
    mut enemy_deck: ResMut<EnemyDeck>,
) {
    let level = r!(level.get()).0;
    let level_config = r!(level_config.get());
    let level_setup = r!(level_config.levels.get(level));
    let deck_config = r!(deck_config.get());

    if level == 0 {
        *player_deck = deck_config.player_decks[0].clone();
    }
    *enemy_deck = level_setup.enemy_deck.clone();
}

fn spawn_level(
    mut commands: Commands,
    game_assets: Res<GameplayAssets>,
    level: NextRef<Level>,
    level_config: ConfigRef<LevelConfig>,
    hud_config: ConfigRef<HudConfig>,
    ship_config: ConfigRef<ShipConfig>,
    player_deck: Res<PlayerDeck>,
    enemy_deck: Res<EnemyDeck>,
) {
    let level = r!(level.get()).0;
    let level_config = r!(level_config.get());
    let level_setup = r!(level_config.levels.get(level));
    let hud_config = r!(hud_config.get());
    let ship_config = r!(ship_config.get());

    commands.spawn(background(&game_assets, level));
    commands.spawn((
        hud::hud(hud_config, &game_assets),
        DespawnOnExitState::<Level>::default(),
    ));
    commands.spawn((
        widget::tiny_label(format!("[b]{}", level_setup.name)),
        Node {
            top: Vw(1.0),
            right: Vw(1.0),
            ..Node::DEFAULT.abs()
        },
        DespawnOnExitState::<Level>::default(),
    ));
    commands.spawn((
        Name::new("Toaster"),
        Toaster,
        Node {
            right: Val::ZERO,
            padding: UiRect::top(Vw(3.0)),
            ..Node::COLUMN.center().size(Vw(74.2), Vw(44.2)).abs()
        },
        DespawnOnExitState::<Level>::default(),
    ));
    commands.spawn((
        player_ship(ship_config, &game_assets, player_deck.max_health),
        Transform::from_xyz(61.0, -46.0, 2.0),
        DespawnOnExitState::<Level>::default(),
    ));
    commands.spawn((
        enemy_ship(ship_config, &game_assets, enemy_deck.max_health),
        Transform::from_xyz(59.0, 93.0, 0.0),
        DespawnOnExitState::<Level>::default(),
    ));
    commands
        .spawn((
            Name::new("EnemyEscapeSensor"),
            Transform::from_xyz(59.0, 240.0, 0.0),
            Sensor,
            Collider::rectangle(400.0, 50.0),
            CollisionLayers::new(GameLayer::Default, GameLayer::Enemy),
            CollisionEventsEnabled,
            DespawnOnExitState::<Level>::default(),
        ))
        .observe(win_level_on_enemy_escape);
}

fn win_level_on_enemy_escape(
    _: Trigger<OnCollisionStart>,
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    game_assets: Res<GameplayAssets>,
    mut menu: ResMut<NextStateStack<Menu>>,
) {
    commands.spawn((
        sfx_audio(&audio_settings, game_assets.ship_death_sfx.clone(), 1.0),
        DespawnOnExitState::<Level>::default(),
    ));

    menu.push(Menu::Upgrade);
    menu.acquire();
}

fn background(game_assets: &GameplayAssets, level: usize) -> impl Bundle {
    (
        Name::new("Background"),
        Sprite::from_image(
            match level {
                0 => &game_assets.bg_level0,
                1 => &game_assets.bg_level1,
                2 => &game_assets.bg_level2,
                3 => &game_assets.bg_level3,
                4 => &game_assets.bg_level4,
                5 => &game_assets.bg_level5,
                6 => &game_assets.bg_level6,
                7 => &game_assets.bg_level7,
                8 => &game_assets.bg_level8,
                9 => &game_assets.bg_level9,
                _ => &game_assets.bg_level0,
            }
            .clone(),
        ),
        Transform::from_xyz(0.0, 0.0, -2.0),
        DespawnOnExitState::<Level>::default(),
        children![(
            Name::new("DimmingOverlay"),
            Sprite::from_color(Color::BLACK.with_alpha(0.7), vec2(520.0, 310.0)),
            Transform::from_xyz(0.0, 0.0, 1.0),
        )],
    )
}
