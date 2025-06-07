use crate::core::audio::AudioSettings;
use crate::core::audio::IsMusicAudio;
use crate::core::audio::music_audio;
use crate::game::GameAssets;
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

    app.configure::<TitleAssets>();
}

fn spawn_title_screen(
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    title_assets: Res<TitleAssets>,
    music_query: Query<(), With<IsMusicAudio>>,
) {
    commands.spawn(background(&title_assets));
    if music_query.is_empty() {
        commands.spawn(music_audio(&audio_settings, title_assets.music.clone()));
    }
}

fn background(title_assets: &TitleAssets) -> impl Bundle {
    (
        Name::new("Background"),
        ImageNode::from(title_assets.background.clone()),
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
    #[asset(path = "image/space/level0.png")]
    pub background: Handle<Image>,
}

impl Configure for TitleAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}
