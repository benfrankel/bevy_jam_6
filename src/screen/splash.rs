use bevy::diagnostic::FrameCount;
use bevy::image::ImageLoaderSettings;
use bevy::image::ImageSampler;

use crate::core::camera::CameraConfig;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::ScreenTime;
use crate::screen::fade::FADE_IN_SECS;
use crate::screen::fade::FadeOut;
use crate::screen::fade::fade_out;
use crate::screen::title::TitleAssets;
use crate::theme::color::ThemeConfig;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Splash.bevy()).load_collection::<TitleAssets>(),
    );
    app.add_systems(StateFlush, Screen::Splash.on_enter(spawn_splash_screen));
    app.add_systems(
        Update,
        Screen::Splash.on_update((
            update_splash,
            CameraConfig::progress.track_progress::<BevyState<Screen>>(),
            ThemeConfig::progress.track_progress::<BevyState<Screen>>(),
        )),
    );
}

const SPLASH_SCREEN_MIN_SECS: f32 = 0.8;

fn spawn_splash_screen(
    mut commands: Commands,
    title_assets: Res<TitleAssets>,
    screen_root: Res<ScreenRoot>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(background(&title_assets));
    commands
        .entity(screen_root.ui)
        .with_child(widget::center(children![(
            Name::new("SplashImage"),
            ImageNode::new(asset_server.load_with_settings(
                // TODO: Workaround for <https://github.com/bevyengine/bevy/issues/14246>.
                //       Use `embedded_asset!` once that's fixed.
                "image/splash.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::linear();
                },
            )),
            Node::DEFAULT.width(Percent(70.0)),
            ThemeColor::BodyText.set::<ImageNode>(),
        )]));
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
        children![widget::dimming_overlay()],
    )
}

fn update_splash(
    mut commands: Commands,
    title_assets: Res<TitleAssets>,
    screen_time: Res<ScreenTime>,
    progress: Res<ProgressTracker<BevyState<Screen>>>,
    frame: Res<FrameCount>,
    fade_out_query: Query<(), With<FadeOut>>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();
    if *last_done != done {
        info!("[Frame {}] Booting: {done} / {total}", frame.0);
    }
    *last_done = done;

    // Continue to the next screen when ready.
    if done == total
        && screen_time.0.as_secs_f32() >= FADE_IN_SECS + SPLASH_SCREEN_MIN_SECS
        && fade_out_query.is_empty()
    {
        commands.spawn(fade_out(&title_assets, Screen::Title));
    }
}
