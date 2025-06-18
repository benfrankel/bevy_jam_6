use bevy::window::ExitCondition;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;
use bevy::window::WindowResolution;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(WindowPlugin {
        primary_window: Some(Window {
            #[cfg(feature = "dev")]
            name: Some("bevy_app".to_string()),
            title: "Flux Pursuit".to_string(),
            mode: WindowMode::Windowed,
            present_mode: PresentMode::AutoVsync,
            resolution: WindowResolution::new(960.0, 540.0).with_scale_factor_override(1.0),
            fit_canvas_to_parent: true,
            ..default()
        }),
        exit_condition: ExitCondition::OnPrimaryClosed,
        ..default()
    });

    app.configure::<WindowRoot>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct WindowRoot {
    pub primary: Entity,
}

impl Configure for WindowRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for WindowRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            primary: world
                .query_filtered::<Entity, With<PrimaryWindow>>()
                .single(world)
                .unwrap(),
        }
    }
}
