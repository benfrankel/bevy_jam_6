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
        app.add_observer(deal_damage_on_collision);
        app.add_observer(reduce_health_on_damage);
    }
}

fn deal_damage_on_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    damage_query: Query<&Damage>,
    health_query: Query<(), With<Health>>,
) {
    let hitbox = r!(trigger.get_target());
    let damage = rq!(damage_query.get(hitbox));

    let hurtbox = trigger.collider;
    rq!(health_query.contains(hurtbox));

    commands.entity(hitbox).try_despawn();
    commands.entity(hurtbox).trigger(OnDamage(damage.0));
}

fn reduce_health_on_damage(trigger: Trigger<OnDamage>, mut health_query: Query<&mut Health>) {
    let target = r!(trigger.get_target());
    r!(health_query.get_mut(target)).current -= trigger.0;
}
