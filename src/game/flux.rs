use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Flux>();
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Flux(pub f32);

impl Configure for Flux {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}
