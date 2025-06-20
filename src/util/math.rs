use crate::prelude::*;

/// The parameters of an exponential approach function on [0, infinity).
#[derive(Reflect, Serialize, Deserialize, Default)]
pub struct Approach {
    /// The initial value.
    pub a: f32,
    /// The value that will be approached asymptotically.
    pub b: f32,
    /// The exponential approach rate.
    pub rate: f32,
}

impl Approach {
    pub fn sample(&self, t: f32) -> f32 {
        (self.a - self.b) * self.rate.powf(t) + self.b
    }
}

/// The parameters of a scaling traumatic event.
#[derive(Reflect, Serialize, Deserialize)]
pub struct ScalingTrauma {
    /// The magnitude of the trauma.
    pub magnitude: f32,
    /// The exponential scaling rate of the trauma.
    pub rate: f32,
    /// The lower bound on the scaling value.
    pub low: f32,
}

impl ScalingTrauma {
    pub fn sample(&self, t: f32) -> f32 {
        self.magnitude * self.rate.powf(self.low.max(t))
    }
}

impl Default for ScalingTrauma {
    fn default() -> Self {
        Self {
            magnitude: 0.0,
            rate: 1.0,
            low: 0.0,
        }
    }
}
