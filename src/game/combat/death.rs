use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<OnDeath>();
}

#[derive(Event, Reflect, Debug)]
pub struct OnDeath;

impl Configure for OnDeath {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(despawn_on_death);
    }
}

fn despawn_on_death(trigger: Trigger<OnDeath>, mut commands: Commands) {
    commands.entity(r!(trigger.get_target())).try_despawn();
}
