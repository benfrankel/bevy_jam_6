use bevy::math::ops::powf;
use crate::animation::offset::NodeOffset;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<NodeShake>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct NodeShake {
    magnitude: Vec2,
    /// The base of an exponent.
    decay: f32,
}

impl NodeShake {
    pub fn new(magnitude: Vec2, decay: f32) -> Self {
        Self {
            magnitude,
            decay,
        }
    }
}

impl Configure for NodeShake {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_shake);
    }
}

fn apply_shake(mut query: Query<(&mut NodeShake, &mut NodeOffset)>) {
    for (mut shake, mut offset) in query.iter_mut().filter(|(shake, _)| shake.magnitude.x + shake.magnitude.y > 0.) {
        let rng = &mut StdRng::from_entropy();
        let point = Rectangle::new(shake.magnitude.x, shake.magnitude.y).sample_interior(rng);

        offset.x = Px(point.x);
        offset.y = Px(point.y);
        shake.magnitude = shake.magnitude * powf(shake.decay, -2.);
    }
}


