use crate::game::deck::DeckConfig;
use crate::game::deck::EnemyDeck;
use crate::game::deck::PlayerDeck;
use crate::game::hud::HudAssets;
use crate::game::hud::hud;
use crate::game::ship::ShipAssets;
use crate::game::ship::ShipConfig;
use crate::game::ship::enemy_ship;
use crate::game::ship::player_ship;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(LevelAssets, Level)>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[asset(path = "image/space/level0.png")]
    bg_level0: Handle<Image>,
    #[asset(path = "image/space/level1.png")]
    bg_level1: Handle<Image>,
    #[asset(path = "image/space/level2.png")]
    bg_level2: Handle<Image>,
    #[asset(path = "image/space/level3.png")]
    bg_level3: Handle<Image>,
    #[asset(path = "image/space/level4.png")]
    bg_level4: Handle<Image>,
    #[asset(path = "image/space/level5.png")]
    bg_level5: Handle<Image>,
    #[asset(path = "image/space/level6.png")]
    bg_level6: Handle<Image>,
    #[asset(path = "image/space/level7.png")]
    bg_level7: Handle<Image>,
    #[asset(path = "image/space/level8.png")]
    bg_level8: Handle<Image>,
    #[asset(path = "image/space/level9.png")]
    bg_level9: Handle<Image>,
}

impl Configure for LevelAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(State, Reflect, Copy, Clone, Default, Eq, PartialEq, Debug)]
#[state(log_flush, react)]
#[reflect(Resource)]
pub struct Level(pub usize);

impl Configure for Level {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(
            StateFlush,
            (
                Level(1).on_enter(set_initial_player_deck),
                Level::ANY.on_edge(reset_decks, spawn_level),
            ),
        );
    }
}

fn set_initial_player_deck(
    deck_config: ConfigRef<DeckConfig>,
    mut player_deck: ResMut<PlayerDeck>,
) {
    let deck_config = r!(deck_config.get());
    *player_deck = deck_config.initial_player_deck();
}

fn reset_decks(mut player_deck: ResMut<PlayerDeck>, mut enemy_deck: ResMut<EnemyDeck>) {
    player_deck.reset();
    enemy_deck.reset();
}

pub fn spawn_level(
    mut commands: Commands,
    ship_config: ConfigRef<ShipConfig>,
    level: NextRef<Level>,
    hud_assets: Res<HudAssets>,
    level_assets: Res<LevelAssets>,
    ship_assets: Res<ShipAssets>,
) {
    let ship_config = r!(ship_config.get());

    commands.spawn(background(&level_assets, level.unwrap().0));
    commands.spawn((hud(&hud_assets), DespawnOnExitState::<Level>::default()));
    commands.spawn((
        player_ship(ship_config, &ship_assets),
        DespawnOnExitState::<Level>::default(),
        Transform::from_xyz(61.0, -46.0, 2.0),
    ));
    commands.spawn((
        enemy_ship(ship_config, &ship_assets),
        DespawnOnExitState::<Level>::default(),
        Transform::from_xyz(59.0, 93.0, 0.0),
    ));
}

fn background(level_assets: &LevelAssets, level: usize) -> impl Bundle {
    (
        Name::new("Background"),
        Sprite::from_image(
            match level {
                0 => &level_assets.bg_level0,
                1 => &level_assets.bg_level1,
                2 => &level_assets.bg_level2,
                3 => &level_assets.bg_level3,
                4 => &level_assets.bg_level4,
                5 => &level_assets.bg_level5,
                6 => &level_assets.bg_level6,
                7 => &level_assets.bg_level7,
                8 => &level_assets.bg_level8,
                9 => &level_assets.bg_level9,
                _ => &level_assets.bg_level0,
            }
            .clone(),
        ),
        Transform::from_xyz(0.0, 0.0, -2.0),
        DespawnOnExitState::<Level>::default(),
        children![(
            Name::new("DimmingOverlay"),
            Sprite::from_color(Color::BLACK.with_alpha(0.7), vec2(480.0, 270.0)),
            Transform::from_xyz(0.0, 0.0, 1.0),
        )],
    )
}
