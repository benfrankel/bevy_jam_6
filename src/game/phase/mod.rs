mod enemy;
pub mod player;
mod reactor;
mod setup;

use crate::game::level::Level;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<PhaseConfig>, Phase, Round, Step, StepTimer)>();
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct PhaseConfig {
    setup_cooldown: f32,
    setup_cooldown_decay: f32,
    setup_first_cooldown: f32,
    setup_last_cooldown: f32,

    reactor_cooldown: f32,
    reactor_cooldown_decay: f32,
    reactor_first_cooldown: f32,
    reactor_last_cooldown: f32,

    enemy_cooldown: f32,
    enemy_cooldown_decay: f32,
    enemy_first_cooldown: f32,
    enemy_last_cooldown: f32,
}

impl Config for PhaseConfig {
    const FILE: &'static str = "phase.ron";
}

#[derive(State, Reflect, Copy, Clone, Default, Eq, PartialEq, Debug)]
#[state(log_flush, after(Level))]
#[reflect(Resource)]
enum Phase {
    #[default]
    Setup,
    Player,
    Reactor,
    Enemy,
}

impl Configure for Phase {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(
            StateFlush,
            Level::ANY.on_edge(Phase::disable, (Phase::enter_default, Phase::trigger)),
        );

        app.add_plugins((
            setup::plugin,
            player::plugin,
            reactor::plugin,
            enemy::plugin,
        ));
    }
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
struct Round(usize);

impl Configure for Round {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(
            StateFlush,
            (
                (Phase::Enemy, Phase::Setup).on_trans(increment_round),
                Phase::ANY.on_disable(reset_round),
            ),
        );
    }
}

fn increment_round(mut round: ResMut<Round>) {
    round.0 += 1;
}

fn reset_round(mut round: ResMut<Round>) {
    round.0 = 0;
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
struct Step(usize);

impl Configure for Step {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(StateFlush, Phase::ANY.on_exit(reset_step));
        app.add_systems(
            Update,
            increment_step
                .in_set(UpdateSystems::SyncLate)
                .run_if(on_step_timer),
        );
    }
}

fn reset_step(mut step: ResMut<Step>) {
    *step = default();
}

fn increment_step(mut step: ResMut<Step>, mut step_timer: ResMut<StepTimer>) {
    step.0 += step_timer.0.times_finished_this_tick() as usize;
    step_timer.0.reset();
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct StepTimer(Timer);

impl Configure for StepTimer {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(
            Update,
            Phase::ANY.on_update(
                tick_step_timer
                    .in_set(UpdateSystems::TickTimers)
                    .in_set(PausableSystems),
            ),
        );
    }
}

fn tick_step_timer(time: Res<Time>, mut step_timer: ResMut<StepTimer>) {
    step_timer.0.tick(time.delta());
}

fn on_step_timer(step_timer: Res<StepTimer>) -> bool {
    step_timer.0.just_finished()
}
