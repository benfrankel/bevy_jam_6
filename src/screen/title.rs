use crate::core::audio::AudioSettings;
use crate::core::audio::MusicAudio;
use crate::core::audio::music_audio;
use crate::game::GameAssets;
use crate::game::combat::damage::DamageConfig;
use crate::game::combat::health::HealthConfig;
use crate::game::deck::DeckConfig;
use crate::game::hud::HudConfig;
use crate::game::level::LevelConfig;
use crate::game::phase::PhaseConfig;
use crate::game::projectile::ProjectileConfig;
use crate::game::ship::ShipConfig;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(Screen::Title.bevy()).load_collection::<GameAssets>());
    app.add_systems(
        StateFlush,
        Screen::Title.on_enter((
            (Menu::Main.enter(), Menu::acquire).chain(),
            spawn_title_screen,
        )),
    );
    app.add_systems(
        Update,
        Screen::Title.on_update((
            DamageConfig::progress.track_progress::<BevyState<Screen>>(),
            DeckConfig::progress.track_progress::<BevyState<Screen>>(),
            HealthConfig::progress.track_progress::<BevyState<Screen>>(),
            HudConfig::progress.track_progress::<BevyState<Screen>>(),
            LevelConfig::progress.track_progress::<BevyState<Screen>>(),
            PhaseConfig::progress.track_progress::<BevyState<Screen>>(),
            ProjectileConfig::progress.track_progress::<BevyState<Screen>>(),
            ShipConfig::progress.track_progress::<BevyState<Screen>>(),
        )),
    );

    app.configure::<TitleAssets>();
}

fn spawn_title_screen(
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    title_assets: Res<TitleAssets>,
    music_query: Query<(), With<MusicAudio>>,
) {
    commands.spawn(background(&title_assets));
    if music_query.is_empty() {
        commands.spawn(music_audio(&audio_settings, title_assets.music.clone()));
    }
}

fn background(title_assets: &TitleAssets) -> impl Bundle {
    (
        Name::new("Background"),
        ImageNode::from(title_assets.background.clone()).with_rect(Rect {
            min: vec2(20.0, 20.0),
            max: vec2(500.0, 290.0),
        }),
        Node::DEFAULT.full_size().abs(),
        GlobalZIndex(-2),
        DespawnOnExitState::<Screen>::Recursive,
        children![widget::rainbow_overlay(), widget::dimming_overlay()],
    )
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TitleAssets {
    #[asset(path = "audio/music/Bevy Jam 6 song 3_5.ogg")]
    music: Handle<AudioSource>,
    #[asset(path = "image/space/space0.png")]
    pub background: Handle<Image>,
}

impl Configure for TitleAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}
