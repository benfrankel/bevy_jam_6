use crate::prelude::*;

/// An exponential approach curve on [0, infinity).
#[derive(Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExponentialApproach {
    /// The value at 0.
    pub a: f32,
    /// The value "at infinity".
    pub b: f32,
    /// The exponential approach rate.
    pub rate: f32,
}

impl Curve<f32> for ExponentialApproach {
    fn domain(&self) -> Interval {
        Interval::new(0.0, f32::INFINITY).unwrap()
    }

    fn sample_unchecked(&self, t: f32) -> f32 {
        (self.a - self.b) * self.rate.powf(t) + self.b
    }
}

/// The parameters of a scaling traumatic event.
#[derive(Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExponentialFit(pub Vec2, pub Vec2);

impl Curve<f32> for ExponentialFit {
    fn domain(&self) -> Interval {
        Interval::new(self.0.x, self.1.x).unwrap()
    }

    fn sample_unchecked(&self, t: f32) -> f32 {
        let r = (self.1.y / self.0.y).powf((self.1.x - self.0.x).recip());
        self.0.y * r.powf(t - self.0.x)
    }
}

impl Default for ExponentialFit {
    fn default() -> Self {
        Self(vec2(0.0, 1.0), vec2(1.0, 1.0))
    }
}
