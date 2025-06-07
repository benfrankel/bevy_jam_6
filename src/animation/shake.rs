use crate::animation::offset::NodeOffset;
use crate::animation::offset::Offset;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Shake, NodeShake, ShakeWithScreen)>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Offset)]
pub struct Shake {
    pub amplitude: Vec2,
    pub trauma: f32,
    pub decay: f32,
    pub exponent: f32,
}

impl Configure for Shake {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_shake);
    }
}

impl Default for Shake {
    fn default() -> Self {
        Self {
            amplitude: Vec2::ZERO,
            trauma: 0.0,
            decay: 0.0,
            exponent: 1.0,
        }
    }
}

fn apply_shake(time: Res<Time>, mut shake_query: Query<(&mut Shake, &mut Offset)>) {
    let rng = &mut thread_rng();
    for (mut shake, mut offset) in &mut shake_query {
        shake.trauma = shake.trauma.clamp(0.0, 1.0);
        cq!(shake.trauma > f32::EPSILON);

        let amplitude = shake.amplitude * shake.trauma.powf(shake.exponent);
        let point = Rectangle::from_size(amplitude).sample_interior(rng);
        offset.0.x = point.x;
        offset.0.y = point.y;

        let decay = shake.decay * time.delta_secs();
        shake.trauma -= decay;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(NodeOffset)]
pub struct NodeShake {
    pub amplitude: Vec2,
    pub trauma: f32,
    pub decay: f32,
    pub exponent: f32,
}

impl Configure for NodeShake {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_node_shake);
    }
}

impl Default for NodeShake {
    fn default() -> Self {
        Self {
            amplitude: Vec2::ZERO,
            trauma: 0.0,
            decay: 0.0,
            exponent: 1.0,
        }
    }
}

fn apply_node_shake(time: Res<Time>, mut shake_query: Query<(&mut NodeShake, &mut NodeOffset)>) {
    let rng = &mut thread_rng();
    for (mut shake, mut offset) in &mut shake_query {
        shake.trauma = shake.trauma.clamp(0.0, 1.0);
        cq!(shake.trauma > f32::EPSILON);

        let amplitude = shake.amplitude * shake.trauma.powf(shake.exponent);
        let point = Rectangle::from_size(amplitude).sample_interior(rng);
        offset.x = Vw(point.x);
        offset.y = Vw(point.y);

        let decay = shake.decay * time.delta_secs();
        shake.trauma -= decay;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ShakeWithScreen;

impl Configure for ShakeWithScreen {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
