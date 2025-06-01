use crate::game::ship::ShipAssets;
use crate::game::ship::enemy_ship;
use crate::game::ship::player_ship;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(LevelAssets, Level)>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct LevelAssets {
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
    #[asset(path = "image/space/level10.png")]
    bg_level10: Handle<Image>,
}

impl Configure for LevelAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

impl LevelAssets {
    fn bg_level(&self, level: usize) -> Option<&Handle<Image>> {
        match level {
            1 => Some(&self.bg_level1),
            2 => Some(&self.bg_level2),
            3 => Some(&self.bg_level3),
            4 => Some(&self.bg_level4),
            5 => Some(&self.bg_level5),
            6 => Some(&self.bg_level6),
            7 => Some(&self.bg_level7),
            8 => Some(&self.bg_level8),
            9 => Some(&self.bg_level9),
            10 => Some(&self.bg_level10),
            _ => None,
        }
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
        app.add_systems(StateFlush, Level::ANY.on_enter(spawn_level));
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn spawn_level(
    mut commands: Commands,
    level: NextRef<Level>,
    level_assets: Res<LevelAssets>,
    ship_assets: Res<ShipAssets>,
) {
    if let Some(bg) = level_assets.bg_level(level.unwrap().0) {
        commands.spawn((
            Name::new("Background"),
            Sprite::from_image(bg.clone()),
            Transform::from_xyz(0.0, 0.0, -2.0),
            DespawnOnExitState::<Level>::default(),
        ));
        commands.spawn((
            Name::new("BackgroundDimming"),
            Sprite::from_color(Color::BLACK.with_alpha(0.7), vec2(480.0, 270.0)),
            Transform::from_xyz(0.0, 0.0, -1.0),
            DespawnOnExitState::<Level>::default(),
        ));
    }

    commands.spawn((
        player_ship(&ship_assets),
        DespawnOnExitState::<Level>::default(),
        Transform::from_xyz(61.0, -44.0, 0.0),
    ));
    commands.spawn((
        enemy_ship(&ship_assets),
        DespawnOnExitState::<Level>::default(),
        Transform::from_xyz(59.0, 93.0, 0.0),
    ));
}
