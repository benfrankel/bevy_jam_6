use crate::animation::offset::NodeOffset;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<NodeShake>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(NodeOffset)]
pub struct NodeShake {
    pub magnitude: Vec2,
    /// The base of an exponent.
    pub decay: f32,
}

impl Configure for NodeShake {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_shake);
    }
}

impl Default for NodeShake {
    fn default() -> Self {
        Self {
            magnitude: Vec2::ZERO,
            decay: 1.0,
        }
    }
}

fn apply_shake(time: Res<Time>, mut shake_query: Query<(&mut NodeShake, &mut NodeOffset)>) {
    let rng = &mut thread_rng();
    for (mut shake, mut offset) in &mut shake_query {
        cq!(shake.magnitude != Vec2::ZERO);

        let point = Rectangle::from_size(shake.magnitude).sample_interior(rng);
        offset.x = Vw(point.x);
        offset.y = Vw(point.y);

        let decay = shake.decay.powf(2.).powf(time.delta_secs());
        shake.magnitude *= decay;
    }
}
