use crate::game::deck::Deck;
use crate::game::hud::HudAssets;
use crate::game::hud::hud;
use crate::game::missile::IsWeapon;
use crate::game::missile::Missile;
use crate::game::missile::start_position;
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

#[derive(State, Reflect, Copy, Clone, Default, Eq, PartialEq, Debug)]
#[state(log_flush, react)]
#[reflect(Resource)]
pub struct Level(pub usize);

impl Configure for Level {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(StateFlush, Level::ANY.on_edge(reset_deck, spawn_level));
    }
}

fn reset_deck(mut deck: ResMut<Deck>) {
    deck.reset();
}

pub fn spawn_level(
    mut commands: Commands,
    level: NextRef<Level>,
    hud_assets: Res<HudAssets>,
    level_assets: Res<LevelAssets>,
    ship_assets: Res<ShipAssets>,
) {
    commands.spawn((hud(&hud_assets), DespawnOnExitState::<Level>::default()));
    commands.spawn(background(&level_assets, level.unwrap().0));
    commands.spawn(player(&ship_assets));
    commands.spawn(enemy(&ship_assets)).observe(
        |
            trigger: Trigger<OnCollisionStart>,
            mut query: Query<(&mut Transform, &mut LinearVelocity, &Missile)>,
            launchers_query: Query<(&GlobalTransform, &IsWeapon)>,
        | {
            if query.contains(trigger.collider) {
                let n = thread_rng().gen_range(0..=1);
                let weapons = launchers_query.iter().collect::<Vec<_>>();

                let (mut transform, mut velocity, _) = r!(query.single_mut());
                transform.translation = start_position(weapons[n].0.translation().x);
                velocity.y = 0.;
            }
        },
    );
}

fn background(level_assets: &LevelAssets, level: usize) -> impl Bundle {
    (
        Name::new("Background"),
        Sprite::from_image(
            match level {
                1 => &level_assets.bg_level1,
                2 => &level_assets.bg_level2,
                3 => &level_assets.bg_level3,
                4 => &level_assets.bg_level4,
                5 => &level_assets.bg_level5,
                6 => &level_assets.bg_level6,
                7 => &level_assets.bg_level7,
                8 => &level_assets.bg_level8,
                9 => &level_assets.bg_level9,
                10 => &level_assets.bg_level10,
                _ => &level_assets.bg_level1,
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

fn player(ship_assets: &ShipAssets) -> impl Bundle {
    (
        player_ship(ship_assets),
        DespawnOnExitState::<Level>::default(),
        Transform::from_xyz(61.0, -46.0, 2.0),
        children![
            (
                IsWeapon,
                // Collider::rectangle(7., 10.),
                Transform::from_xyz(-10., 11., 0.),
            ),
            (
                IsWeapon,
                // Collider::rectangle(7., 10.),
                Transform::from_xyz(10., 11., 0.),
            ),
        ],
    )
}

fn enemy(ship_assets: &ShipAssets) -> impl Bundle {
    (
        enemy_ship(ship_assets),
        DespawnOnExitState::<Level>::default(),
        Transform::from_xyz(59.0, 93.0, 0.0),
        CollisionEventsEnabled,
    )
}
