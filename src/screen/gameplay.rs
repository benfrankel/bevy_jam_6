use crate::core::audio::AudioSettings;
use crate::core::audio::music_audio;
use crate::game::hud::HudAssets;
use crate::game::level::Level;
use crate::game::level::LevelAssets;
use crate::game::projectile::ProjectileAssets;
use crate::game::ship::ShipAssets;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        Screen::Gameplay.on_edge(
            Level::disable,
            (spawn_gameplay_screen, (Level(0).enter(), Level::trigger)),
        ),
    );

    app.configure::<(GameplayAssets, GameplayAction)>();
}

fn spawn_gameplay_screen(
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    assets: Res<GameplayAssets>,
) {
    commands.spawn((
        music_audio(&audio_settings, assets.music.clone()),
        DespawnOnExitState::<Screen>::Recursive,
    ));
}

pub fn load_collections(state: LoadingState<BevyState<Screen>>) -> LoadingState<BevyState<Screen>> {
    state
        .load_collection::<GameplayAssets>()
        .load_collection::<HudAssets>()
        .load_collection::<LevelAssets>()
        .load_collection::<ProjectileAssets>()
        .load_collection::<ShipAssets>()
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
struct GameplayAssets {
    #[asset(path = "audio/music/545458__bertsz__bit-forest-evil-theme-music.ogg")]
    music: Handle<AudioSource>,
}

impl Configure for GameplayAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameplayAction {
    Pause,
    CloseMenu,
    ToggleTooltips,
}

impl Configure for GameplayAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::Pause, GamepadButton::Start)
                .with(Self::Pause, KeyCode::Escape)
                .with(Self::Pause, KeyCode::KeyP)
                .with(Self::CloseMenu, KeyCode::KeyP)
                .with(Self::ToggleTooltips, KeyCode::KeyI),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            Screen::Gameplay.on_update((
                (spawn_pause_overlay, Menu::Pause.enter())
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_disabled.and(action_just_pressed(Self::Pause))),
                Menu::clear
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_enabled.and(action_just_pressed(Self::CloseMenu))),
                toggle_tooltips
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::ToggleTooltips)),
            )),
        );
    }
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        widget::blocking_overlay(1),
        ThemeColor::Overlay.set::<BackgroundColor>(),
        DespawnOnExitState::<Screen>::default(),
        DespawnOnDisableState::<Menu>::default(),
    ));
}

fn toggle_tooltips(mut tooltip_settings: ResMut<TooltipSettings>) {
    tooltip_settings.enabled ^= true;
}
