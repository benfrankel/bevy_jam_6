use bevy::render::camera::ScalingMode;
use bevy::render::camera::Viewport;
use bevy::window::PrimaryWindow;
use bevy::window::WindowResized;
use bevy::window::WindowScaleFactorChanged;

use crate::animation::shake::Shake;
use crate::animation::shake::ShakeRotation;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<CameraConfig>,
        CameraRoot,
        SmoothFollow,
        Letterbox,
    )>();
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CameraConfig {
    scaling_mode: ScalingMode,
    zoom: f32,
}

impl Config for CameraConfig {
    const FILE: &'static str = "camera.ron";

    fn on_load(&mut self, world: &mut World) {
        let camera = r!(world.get_resource::<CameraRoot>()).primary;
        let projection = r!(world.query::<&mut Projection>().get_mut(world, camera));

        let projection = r!(match projection.into_inner() {
            Projection::Orthographic(x) => Some(x),
            _ => None,
        });
        projection.scale = self.zoom.recip();
        projection.scaling_mode = self.scaling_mode;
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CameraRoot {
    pub primary: Entity,
}

impl Configure for CameraRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for CameraRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            primary: world
                .spawn((
                    Name::new("PrimaryCamera"),
                    IsDefaultUiCamera,
                    Camera2d,
                    Projection::Orthographic(OrthographicProjection {
                        near: -1000.0,
                        ..OrthographicProjection::default_2d()
                    }),
                    Msaa::Off,
                    Letterbox(16.0 / 9.0),
                    SmoothFollow {
                        target: Entity::PLACEHOLDER,
                        rate: Vec2::splat(100.0),
                    },
                    Shake::default(),
                    ShakeRotation::default(),
                ))
                .id(),
        }
    }
}

/// Follow a target entity smoothly.
///
/// This component should only be used on root entities.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct SmoothFollow {
    target: Entity,
    rate: Vec2,
}

impl Configure for SmoothFollow {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_smooth_follow.in_set(PausableSystems));
    }
}

fn apply_smooth_follow(
    time: Res<Time>,
    mut follow_query: Query<(&mut Transform, &SmoothFollow)>,
    target_query: Query<&GlobalTransform, Without<SmoothFollow>>,
) {
    let dt = time.delta_secs();
    for (mut transform, follow) in &mut follow_query {
        let target_pos = cq!(target_query.get(follow.target)).translation().xy();
        let mut pos = transform.translation.xy();
        pos += (target_pos - pos) * (follow.rate * dt).clamp(Vec2::ZERO, Vec2::ONE);
        transform.translation = pos.extend(transform.translation.z);
    }
}

/// Letterbox a camera's viewport to a particular aspect ratio.
#[derive(Component, Clone)]
pub struct Letterbox(pub f32);

impl Configure for Letterbox {
    fn configure(app: &mut App) {
        app.add_systems(
            PostUpdate,
            apply_letterbox
                .run_if(on_event::<WindowResized>.or(on_event::<WindowScaleFactorChanged>)),
        );
    }
}

fn apply_letterbox(
    mut letterbox_query: Query<(&mut Camera, &Letterbox)>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
) {
    for (mut camera, letterbox) in &mut letterbox_query {
        let window_width = primary_window.physical_width() as f32;
        let window_height = primary_window.physical_height() as f32;
        let mut size = vec2(window_width, window_height);
        let mut pos = Vec2::ZERO;

        if window_width / window_height > letterbox.0 {
            size.x = size.y * letterbox.0;
            pos.x = window_width / 2.0 - size.x / 2.0;
        } else {
            size.y = size.x / letterbox.0;
            pos.y = window_height / 2.0 - size.y / 2.0;
        }

        camera.viewport = Some(Viewport {
            physical_position: pos.as_uvec2(),
            physical_size: size.as_uvec2(),
            ..default()
        });
    }
}
