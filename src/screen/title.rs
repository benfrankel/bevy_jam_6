use crate::core::audio::AudioSettings;
use crate::core::audio::music_audio;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::background;
use crate::screen::gameplay::load_collections;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(load_collections(LoadingState::new(Screen::Title.bevy())));
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
) {
    commands.spawn(background(&title_assets));
    commands.spawn((
        music_audio(&audio_settings, title_assets.music.clone()),
        DespawnOnExitState::<Screen>::Recursive,
    ));
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TitleAssets {
    #[asset(path = "audio/music/Bevy Jam 6 song 3_2.ogg")]
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
