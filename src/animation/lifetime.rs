use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Lifetime>();
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Lifetime(pub f32);

impl Configure for Lifetime {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            update_lifetime
                .in_set(UpdateSystems::TickTimers)
                .in_set(PausableSystems),
        );
    }
}

fn update_lifetime(time: Res<Time>, mut lifetime_query: Query<&mut Lifetime>) {
    for mut lifetime in &mut lifetime_query {
        lifetime.0 += time.delta_secs();
    }
}
