use crate::core::dev::DevConfig;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenTime;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<StateDebugSettings>();

    // Extend loading screen.
    app.add_systems(
        Update,
        (
            state!(Screen::Title | Screen::Loading)
                .on_update(force_loading_screen.track_progress::<BevyState<Screen>>()),
            Screen::Loading.on_update(extend_loading_screen.track_progress::<BevyState<Screen>>()),
        ),
    );
}

pub(super) fn on_load(config: &DevConfig, world: &mut World) {
    r!(world.get_resource_mut::<StateDebugSettings>()).log_flush = config.log_state_flush;
    if let Some(screen) = config.initial_screen {
        r!(world.get_resource_mut::<NextStateBuffer<Screen>>()).enter(screen);
    }
}

fn force_loading_screen(config: ConfigRef<DevConfig>, screen: CurrentRef<Screen>) -> Progress {
    let config = r!(config.get());
    (config.extend_loading_screen <= 0.0 || screen.is_in(&Screen::Loading)).into()
}

fn extend_loading_screen(config: ConfigRef<DevConfig>, screen_time: Res<ScreenTime>) -> Progress {
    let config = r!(config.get());
    (screen_time.0.as_secs_f32() >= config.extend_loading_screen).into()
}
