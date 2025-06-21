use crate::animation::PostTransformSystems;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Trauma, Shake, NodeShake)>();
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
#[require(Trauma)]
pub struct Shake {
    pub amplitude: Vec2,
    pub decay: f32,
    pub exponent: f32,
}

impl Configure for Shake {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(PostUpdate, apply_shake.in_set(PostTransformSystems::Blend));
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

fn apply_shake(time: Res<Time>, mut shake_query: Query<(&Shake, &mut Trauma, &mut Transform)>) {
    let rng = &mut thread_rng();
    for (shake, mut trauma, mut transform) in &mut shake_query {
        trauma.0 = trauma.0.clamp(0.0, 1.0);
        cq!(trauma.0 > f32::EPSILON);

        let amplitude = shake.amplitude * trauma.0.powf(shake.exponent);
        let point = Rectangle::from_size(amplitude).sample_interior(rng);
        transform.translation += point.extend(0.0);

        trauma.0 -= shake.decay * time.delta_secs();
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Copy, Clone)]
#[reflect(Component)]
#[require(Trauma)]
pub struct NodeShake {
    pub amplitude_x: Val,
    pub amplitude_y: Val,
    pub decay: f32,
    pub exponent: f32,
}

impl Configure for NodeShake {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            PostUpdate,
            apply_node_shake.in_set(PostTransformSystems::Blend),
        );
    }
}

impl Default for NodeShake {
    fn default() -> Self {
        Self {
            amplitude_x: Val::ZERO,
            amplitude_y: Val::ZERO,
            decay: 0.0,
            exponent: 1.0,
        }
    }
}

fn apply_node_shake(
    time: Res<Time>,
    mut shake_query: Query<(
        &NodeShake,
        &ComputedNode,
        &ComputedNodeTarget,
        &mut Trauma,
        &mut Transform,
    )>,
) {
    let rng = &mut thread_rng();
    for (shake, node, target, mut trauma, mut transform) in &mut shake_query {
        trauma.0 = trauma.0.clamp(0.0, 1.0);
        cq!(trauma.0 > f32::EPSILON);

        let parent_size = node.size().x;
        let target_size = target.physical_size().as_vec2();
        let amplitude_x = match shake.amplitude_x {
            Val::Auto => 0.0,
            x => c!(x.resolve(parent_size, target_size)),
        };
        let amplitude_y = match shake.amplitude_y {
            Val::Auto => 0.0,
            y => c!(y.resolve(parent_size, target_size)),
        };
        let amplitude = vec2(amplitude_x, amplitude_y) * trauma.0.powf(shake.exponent);
        let point = Rectangle::from_size(amplitude).sample_interior(rng);
        transform.translation += point.extend(0.0);

        trauma.0 -= shake.decay * time.delta_secs();
    }
}
