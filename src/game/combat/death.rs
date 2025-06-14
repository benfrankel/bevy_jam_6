use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(OnDeath, Dead)>();
}

#[derive(Event, Reflect, Debug)]
pub struct OnDeath;

impl Configure for OnDeath {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Dead;

impl Configure for Dead {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(mark_dead_on_death);
    }
}

fn mark_dead_on_death(trigger: Trigger<OnDeath>, mut commands: Commands) {
    let target = rq!(trigger.get_target());
    commands.entity(target).try_insert(Dead);
}
