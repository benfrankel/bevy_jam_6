use crate::animation::offset::NodeOffset;
use crate::animation::offset::Offset;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Trauma, Shake, NodeShake, ShakeWithCamera)>();
}

/// See [`Shake`] and [`NodeShake`].
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Trauma(pub f32);

impl Configure for Trauma {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Copy, Clone)]
#[reflect(Component)]
#[require(Trauma, Offset)]
pub struct Shake {
    pub amplitude: Vec2,
    pub decay: f32,
    pub exponent: f32,
}

impl Configure for Shake {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_shake.in_set(UpdateSystems::SyncLate));
    }
}

impl Default for Shake {
    fn default() -> Self {
        Self {
            amplitude: Vec2::ZERO,
            decay: 0.0,
            exponent: 1.0,
        }
    }
}

fn apply_shake(time: Res<Time>, mut shake_query: Query<(&Shake, &mut Trauma, &mut Offset)>) {
    let rng = &mut thread_rng();
    for (shake, mut trauma, mut offset) in &mut shake_query {
        trauma.0 = trauma.0.clamp(0.0, 1.0);
        cq!(trauma.0 > f32::EPSILON);

        let amplitude = shake.amplitude * trauma.0.powf(shake.exponent);
        let point = Rectangle::from_size(amplitude).sample_interior(rng);
        offset.0.x = point.x;
        offset.0.y = point.y;

        let decay = shake.decay * time.delta_secs();
        trauma.0 -= decay;
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Copy, Clone)]
#[reflect(Component)]
#[require(Trauma, NodeOffset)]
pub struct NodeShake {
    pub amplitude: Vec2,
    pub decay: f32,
    pub exponent: f32,
}

impl Configure for NodeShake {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_node_shake.in_set(UpdateSystems::SyncLate));
    }
}

impl Default for NodeShake {
    fn default() -> Self {
        Self {
            amplitude: Vec2::ZERO,
            decay: 0.0,
            exponent: 1.0,
        }
    }
}

fn apply_node_shake(
    time: Res<Time>,
    mut shake_query: Query<(&NodeShake, &mut Trauma, &mut NodeOffset)>,
) {
    let rng = &mut thread_rng();
    for (shake, mut trauma, mut offset) in &mut shake_query {
        trauma.0 = trauma.0.clamp(0.0, 1.0);
        cq!(trauma.0 > f32::EPSILON);

        let amplitude = shake.amplitude * trauma.0.powf(shake.exponent);
        let point = Rectangle::from_size(amplitude).sample_interior(rng);
        offset.x = Vw(point.x);
        offset.y = Vw(point.y);

        let decay = shake.decay * time.delta_secs();
        trauma.0 -= decay;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ShakeWithCamera;

impl Configure for ShakeWithCamera {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
