use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Health>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn diff(&self) -> f32 {
        self.max - self.current
    }
}

impl Configure for Health {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

impl Default for Health {
    fn default() -> Self {
        Health {
            max: 100.,
            current: 90.,
        }
    }
}
