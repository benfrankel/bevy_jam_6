use crate::game::combat::health::Health;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Damage, OnDamage)>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Damage(pub f32);

impl Configure for Damage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Event, Reflect, Debug)]
pub struct OnDamage(pub f32);

impl Configure for OnDamage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(reduce_health_on_damage);
    }
}

fn reduce_health_on_damage(trigger: Trigger<OnDamage>, mut health_query: Query<&mut Health>) {
    let target = r!(trigger.get_target());
    r!(health_query.get_mut(target)).current -= trigger.0;
}
