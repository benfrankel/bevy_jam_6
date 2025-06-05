use crate::animation::lifetime::Lifetime;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Oscillate>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Lifetime)]
pub struct Oscillate {
    pub amplitude: Vec2,
    pub phase: Vec2,
    pub rate: Vec2,
    applied: Vec2,
}

impl Configure for Oscillate {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_oscillate.in_set(UpdateSystems::Update));
    }
}

impl Oscillate {
    pub fn new(amplitude: Vec2, phase: Vec2, rate: Vec2) -> Self {
        Self {
            amplitude,
            phase,
            rate,
            applied: Vec2::ZERO,
        }
    }
}

fn apply_oscillate(mut oscillate_query: Query<(&mut Transform, &mut Oscillate, &Lifetime)>) {
    for (mut transform, mut oscillate, lifetime) in &mut oscillate_query {
        let t = oscillate.phase + oscillate.rate * lifetime.0;
        let offset = oscillate.amplitude * vec2(t.x.sin(), t.y.sin());
        let delta = offset - oscillate.applied;

        oscillate.applied = offset;
        transform.translation += delta.extend(0.0);
    }
}
