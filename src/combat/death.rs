use crate::animation::lifetime::Lifetime;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(OnDeath, Dead, DieOnLifetime, DieOnClick, DespawnOnDeath)>();
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

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Lifetime)]
pub struct DieOnLifetime(pub f32);

impl Configure for DieOnLifetime {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, die_on_lifetime.in_set(UpdateSystems::Update));
    }
}

fn die_on_lifetime(
    mut late: LateCommands,
    despawn_query: Query<(Entity, &DieOnLifetime, &Lifetime)>,
) {
    for (entity, despawn, lifetime) in &despawn_query {
        cq!(despawn.0 <= lifetime.0);
        late.commands().entity(entity).despawn();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct DieOnClick;

impl Configure for DieOnClick {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(die_on_click);
    }
}

fn die_on_click(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    despawn_query: Query<(), With<DieOnClick>>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    let target = rq!(trigger.get_target());
    rq!(despawn_query.contains(target));
    commands.entity(target).despawn();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct DespawnOnDeath;

impl Configure for DespawnOnDeath {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(despawn_on_death);
    }
}

fn despawn_on_death(
    trigger: Trigger<OnDeath>,
    mut late: LateCommands,
    despawn_query: Query<(), With<DespawnOnDeath>>,
) {
    let target = rq!(trigger.get_target());
    rq!(despawn_query.contains(target));
    late.commands().entity(target).despawn();
}
