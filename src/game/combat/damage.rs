use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Damage>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Damage(pub f32);

impl Configure for Damage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
