use crate::animation::PostTransformSystems;
use crate::animation::backup::Backup;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Shake, ShakeRotation, NodeShake)>();
}

/// Translational shake.
#[derive(Component, Reflect, Serialize, Deserialize, Copy, Clone)]
#[reflect(Component)]
#[require(Backup<Transform>)]
#[serde(deny_unknown_fields)]
pub struct Shake {
    pub amplitude: Vec2,
    pub decay: f32,
    pub exponent: f32,
    pub frequency: f32,
    #[serde(default)]
    pub trauma: f32,
}

impl Configure for Shake {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            PostUpdate,
            apply_shake
                .in_set(PostTransformSystems::Blend)
                .in_set(PausableSystems),
        );
    }
}

impl Default for Shake {
    fn default() -> Self {
        Self {
            amplitude: Vec2::ZERO,
            decay: 0.0,
            exponent: 1.0,
            frequency: 1.0,
            trauma: 0.0,
        }
    }
}

fn apply_shake(time: Res<Time>, mut shake_query: Query<(&mut Shake, &mut Transform)>) {
    let noise_fn = Noise::<(
        MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>,
        SNormToUNorm,
    )>::default();
    for (mut shake, mut transform) in &mut shake_query {
        shake.trauma = shake.trauma.clamp(0.0, 1.0);
        cq!(shake.trauma > f32::EPSILON);

        let t = shake.frequency * time.elapsed_secs();
        let noise = vec2(
            noise_fn.sample(vec2(t, 0.5)),
            noise_fn.sample(vec2(t + 100.0, 0.5)),
        );
        let noise = 1.0 - 2.0 * noise;
        let offset = shake.amplitude * shake.trauma.powf(shake.exponent) * noise;
        transform.translation += offset.extend(0.0);

        shake.trauma -= shake.decay * time.delta_secs();
    }
}

/// Rotational shake.
#[derive(Component, Reflect, Serialize, Deserialize, Copy, Clone)]
#[reflect(Component)]
#[require(Backup<Transform>)]
#[serde(deny_unknown_fields)]
pub struct ShakeRotation {
    /// The maximum rotation offset in degrees.
    pub amplitude: f32,
    pub decay: f32,
    pub exponent: f32,
    pub frequency: f32,
    #[serde(default)]
    pub trauma: f32,
}

impl Configure for ShakeRotation {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            PostUpdate,
            apply_shake_rotation
                .in_set(PostTransformSystems::Blend)
                .in_set(PausableSystems),
        );
    }
}

impl Default for ShakeRotation {
    fn default() -> Self {
        Self {
            amplitude: 0.0,
            decay: 0.0,
            exponent: 1.0,
            frequency: 1.0,
            trauma: 0.0,
        }
    }
}

fn apply_shake_rotation(
    time: Res<Time>,
    mut shake_query: Query<(&mut ShakeRotation, &mut Transform)>,
) {
    let noise_fn = Noise::<(
        MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>,
        SNormToUNorm,
    )>::default();
    for (mut shake, mut transform) in &mut shake_query {
        shake.trauma = shake.trauma.clamp(0.0, 1.0);
        cq!(shake.trauma > f32::EPSILON);

        let t = shake.frequency * time.elapsed_secs();
        let noise: f32 = noise_fn.sample(vec2(t + 200.0, 0.5));
        let noise = 1.0 - 2.0 * noise;
        let offset = shake.amplitude * shake.trauma.powf(shake.exponent) * noise;
        transform.rotate_z(offset.to_radians());

        shake.trauma -= shake.decay * time.delta_secs();
    }
}

/// Translational shake with amplitude defined in terms of [`Val`].
#[derive(Component, Reflect, Serialize, Deserialize, Copy, Clone)]
#[reflect(Component)]
#[require(Backup<Transform>)]
#[serde(deny_unknown_fields)]
pub struct NodeShake {
    pub amplitude_x: Val,
    pub amplitude_y: Val,
    pub decay: f32,
    pub exponent: f32,
    pub frequency: f32,
    #[serde(default)]
    pub trauma: f32,
}

impl Configure for NodeShake {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            PostUpdate,
            apply_node_shake
                .in_set(PostTransformSystems::Blend)
                .in_set(PausableSystems),
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
            frequency: 1.0,
            trauma: 0.0,
        }
    }
}

fn apply_node_shake(
    time: Res<Time>,
    mut shake_query: Query<(
        &mut NodeShake,
        &ComputedNode,
        &ComputedNodeTarget,
        &mut Transform,
    )>,
) {
    let noise_fn = Noise::<(
        MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>,
        SNormToUNorm,
    )>::default();
    for (mut shake, node, target, mut transform) in &mut shake_query {
        shake.trauma = shake.trauma.clamp(0.0, 1.0);
        cq!(shake.trauma > f32::EPSILON);

        // Resolve amplitude Vals.
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
        let amplitude = vec2(-amplitude_x, amplitude_y);

        let t = shake.frequency * time.elapsed_secs();
        let noise = vec2(
            noise_fn.sample(vec2(t, 0.5)),
            noise_fn.sample(vec2(t + 100.0, 0.5)),
        );
        let noise = 1.0 - 2.0 * noise;
        let offset = amplitude * shake.trauma.powf(shake.exponent) * noise;
        transform.translation += offset.extend(0.0);

        shake.trauma -= shake.decay * time.delta_secs();
    }
}
