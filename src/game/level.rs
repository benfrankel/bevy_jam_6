use crate::animation::shake::Shake;
use crate::core::camera::CameraRoot;
use crate::game::GameAssets;
use crate::game::deck::DeckConfig;
use crate::game::deck::EnemyDeck;
use crate::game::deck::PlayerDeck;
use crate::game::hud::hud;
use crate::game::module::Module;
use crate::game::ship::ShipConfig;
use crate::game::ship::enemy_ship;
use crate::game::ship::player_ship;
use crate::menu::Menu;
use crate::prelude::*;

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
    pub enemy_deck: EnemyDeck,
    pub enemy_health: f32,
    pub reward_max_health: f32,
    pub reward_heat_capacity: f32,
    pub reward_reactor_slots: usize,
    // pub fixed_rewards: Vec<LevelReward>,
}

#[derive(Reflect, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum LevelReward {
    MaxHealth(f32),
    HeatCapacity(f32),
    ReactorSlots(usize),
    Module(Module),
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
                (reset_decks, reset_camera),
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

fn reset_decks(mut player_deck: ResMut<PlayerDeck>, mut enemy_deck: ResMut<EnemyDeck>) {
    player_deck.reset();
    player_deck.shuffle(&mut thread_rng());
    enemy_deck.reset();
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
        *player_deck = deck_config.initial_player_deck.clone();
    }
    *enemy_deck = level_setup.enemy_deck.clone();
}

fn spawn_level(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    level: NextRef<Level>,
    level_config: ConfigRef<LevelConfig>,
    ship_config: ConfigRef<ShipConfig>,
    player_deck: Res<PlayerDeck>,
) {
    let level = r!(level.get()).0;
    let level_config = r!(level_config.get());
    let level_setup = r!(level_config.levels.get(level));
    let ship_config = r!(ship_config.get());

    commands.spawn(background(&game_assets, level));
    commands.spawn((hud(&game_assets), DespawnOnExitState::<Level>::default()));
    commands.spawn((
        player_ship(ship_config, &game_assets, player_deck.max_health),
        DespawnOnExitState::<Level>::default(),
        Transform::from_xyz(61.0, -46.0, 2.0),
    ));
    commands.spawn((
        enemy_ship(ship_config, &game_assets, level_setup.enemy_health),
        DespawnOnExitState::<Level>::default(),
        Transform::from_xyz(59.0, 93.0, 0.0),
    ));
}

fn background(game_assets: &GameAssets, level: usize) -> impl Bundle {
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
