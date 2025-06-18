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
    pub fn eval(&self, t: f32) -> f32 {
        (self.a - self.b) * self.rate.powf(t) + self.b
    }
}
