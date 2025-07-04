use crate::animation::PostColorSystems;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::title::TitleAssets;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ScreenFadeIn, ScreenFadeOut)>();
}

pub const FADE_IN_SECS: f32 = 0.8;
const FADE_OUT_SECS: f32 = 0.8;
const FADE_OUT_PAUSE_SECS: f32 = 0.5;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ScreenFadeIn {
    duration: f32,
    remaining: f32,
}

impl Configure for ScreenFadeIn {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(PostUpdate, apply_fade_in.in_set(PostColorSystems::Blend));
        app.add_systems(StateFlush, Screen::ANY.on_enter(spawn_fade_in));
    }
}

impl ScreenFadeIn {
    fn new(duration: f32) -> Self {
        Self {
            duration,
            remaining: duration,
        }
    }
}

fn apply_fade_in(
    time: Res<Time>,
    mut late: LateCommands,
    mut fade_query: Query<(Entity, &mut ScreenFadeIn, &mut ImageNode)>,
) {
    let dt = time.delta_secs();
    for (entity, mut fade, mut image) in &mut fade_query {
        let t = fade.remaining / fade.duration;
        let curve = ExponentialApproach {
            a: 0.0,
            b: 1.0,
            rate: 0.001,
        };
        image.color.set_alpha(curve.sample_clamped(t));
        if fade.remaining <= 0.0 {
            late.commands().entity(entity).despawn();
        }
        fade.remaining -= dt;
    }
}

fn spawn_fade_in(mut commands: Commands, title_assets: Res<TitleAssets>) {
    commands.spawn(fade_in(&title_assets));
}

/// A screen transition animation for entering the current [`Screen`].
pub fn fade_in(title_assets: &TitleAssets) -> impl Bundle {
    (
        widget::nonblocking_overlay(1000),
        ScreenFadeIn::new(FADE_IN_SECS),
        ImageNode::from(title_assets.background.clone())
            .with_rect(Rect {
                min: vec2(20.0, 20.0),
                max: vec2(500.0, 290.0),
            })
            .with_color(Color::srgb(0.8, 0.8, 0.8)),
    )
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScreenFadeOut {
    duration: f32,
    remaining: f32,
    to_screen: Screen,
}

impl Configure for ScreenFadeOut {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(PostUpdate, apply_fade_out.in_set(PostColorSystems::Blend));
    }
}

impl ScreenFadeOut {
    fn new(duration: f32, to_screen: Screen) -> Self {
        Self {
            duration,
            remaining: duration,
            to_screen,
        }
    }
}

fn apply_fade_out(
    time: Res<Time>,
    mut late: LateCommands,
    mut screen: NextMut<Screen>,
    mut fade_query: Query<(Entity, &mut ScreenFadeOut, &mut ImageNode)>,
) {
    let dt = time.delta_secs();
    for (entity, mut fade, mut image) in &mut fade_query {
        let t = ((fade.remaining - FADE_OUT_PAUSE_SECS) / (fade.duration - FADE_OUT_PAUSE_SECS))
            .clamp(0.0, 1.0);
        let curve = ExponentialApproach {
            a: 0.0,
            b: 1.0,
            rate: 0.001,
        };
        image.color.set_alpha(curve.sample_clamped(1.0 - t));
        if fade.remaining <= 0.0 {
            screen.trigger().enter(fade.to_screen);
            late.commands().entity(entity).despawn();
        }
        fade.remaining -= dt;
    }
}

/// A screen transition animation for exiting the current [`Screen`].
pub fn fade_out(title_assets: &TitleAssets, to_screen: Screen) -> impl Bundle {
    (
        widget::blocking_overlay(1000),
        ScreenFadeOut::new(FADE_OUT_SECS + FADE_OUT_PAUSE_SECS, to_screen),
        ImageNode::from(title_assets.background.clone())
            .with_rect(Rect {
                min: vec2(20.0, 20.0),
                max: vec2(500.0, 290.0),
            })
            .with_color(Color::srgb(0.8, 0.8, 0.8)),
    )
}
