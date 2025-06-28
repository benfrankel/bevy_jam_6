use crate::animation::PostColorSystems;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<FadeOut>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct FadeOut {
    pub initial_alpha: f32,
    pub duration: f32,
    pub remaining: f32,
}

impl Configure for FadeOut {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            PostUpdate,
            apply_fade_out
                .in_set(PostColorSystems::Blend)
                .in_set(PausableSystems),
        );
    }
}

impl FadeOut {
    pub fn new(duration: f32) -> Self {
        Self {
            initial_alpha: 1.0,
            duration,
            remaining: duration,
        }
    }
}

fn apply_fade_out(
    mut commands: Commands,
    time: Res<Time>,
    mut fade_query: Query<(Entity, &mut FadeOut)>,
    children_query: Query<&Children>,
    mut color_query: Query<(
        Option<&mut BackgroundColor>,
        Option<&mut BorderColor>,
        Option<&mut BoxShadow>,
        Option<&mut TextColor>,
    )>,
) {
    let curve = EasingCurve::new(1.0, 0.0, EaseFunction::QuadraticOut);
    for (entity, mut fade) in &mut fade_query {
        fade.remaining -= time.delta_secs();
        if fade.remaining < f32::EPSILON {
            commands.entity(entity).try_despawn();
            continue;
        }

        let alpha = fade.initial_alpha * curve.sample_clamped(1.0 - fade.remaining / fade.duration);
        for entity in std::iter::once(entity).chain(children_query.iter_descendants(entity)) {
            let (maybe_background_color, maybe_border_color, maybe_box_shadow, maybe_text_color) =
                cq!(color_query.get_mut(entity));

            if let Some(mut color) = maybe_background_color {
                color.0 = color.0.with_alpha(alpha * color.0.alpha());
            }
            if let Some(mut color) = maybe_border_color {
                color.0 = color.0.with_alpha(alpha * color.0.alpha());
            }
            if let Some(mut shadows) = maybe_box_shadow {
                for shadow in &mut shadows.0 {
                    shadow.color = shadow.color.with_alpha(alpha * shadow.color.alpha());
                }
            }
            if let Some(mut color) = maybe_text_color {
                color.0 = color.0.with_alpha(alpha * color.0.alpha());
            }
        }
    }
}
