use crate::game::level::Level;
use crate::game::phase::Phase;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Stats>();
    app.init_resource::<Stats>();
    app.add_systems(StateFlush, Phase::Player.on_enter(increment_level_rounds));
    app.add_systems(
        StateFlush,
        Level::ANY.on_exit(calculate_average_turns_per_round),
    );
}

#[derive(Resource, Reflect, Debug, Default, Clone)]
#[reflect(Resource)]
pub struct Stats {
    pub missiles_fired: usize,
    pub lasers_fired: usize,
    pub fireballs_unleashed: usize,
    pub total_repaired: f32,
    pub highest_flux: f32,
    pub longest_chain: f32,
    pub average_rounds_per_level: f32,
    pub damage_given: f32,
    pub damage_taken: f32,
    pub highest_damage: f32,

    num_rounds_buffer: f32,
}

fn increment_level_rounds(mut stats: ResMut<Stats>) {
    stats.num_rounds_buffer += 1.0;
}

fn calculate_average_turns_per_round(mut stats: ResMut<Stats>) {
    if stats.average_rounds_per_level > 0.0 {
        stats.average_rounds_per_level =
            (stats.average_rounds_per_level + stats.num_rounds_buffer) / 2.0;
    } else {
        stats.average_rounds_per_level = stats.num_rounds_buffer;
    }

    stats.num_rounds_buffer = 0.0;
}

pub fn stats_grid(stats: Res<Stats>, level: String) -> impl Bundle {
    let stats = stats.clone();
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            margin: UiRect::top(Vw(2.0)).with_bottom(Vw(5.2)),
            row_gap: Vw(1.4),
            column_gap: Vw(6.0),
            grid_template_columns: vec![
                RepeatedGridTrack::flex(1, 6.0),
                RepeatedGridTrack::flex(1, 1.0),
                RepeatedGridTrack::flex(1, 6.0),
                RepeatedGridTrack::flex(1, 1.0),
            ],
            ..default()
        },
        GridAlignment::columns([JustifySelf::End, JustifySelf::Start]),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent.spawn(widget::small_label("[b]Levels Completed:"));
            parent.spawn(widget::small_label(level));
            parent.spawn(widget::small_label("[b]Missiles Fired:"));
            parent.spawn(widget::small_label(stats.missiles_fired.to_string()));
            parent.spawn(widget::small_label("[b]Lasers Fired:"));
            parent.spawn(widget::small_label(stats.lasers_fired.to_string()));
            parent.spawn(widget::small_label("[b]Fireballs Unleashed:"));
            parent.spawn(widget::small_label(stats.fireballs_unleashed.to_string()));
            parent.spawn(widget::small_label("[b]Total Repaired:"));
            parent.spawn(widget::small_label(stats.total_repaired.to_string()));
            parent.spawn(widget::small_label("[b]Damage Given:"));
            parent.spawn(widget::small_label(stats.damage_given.to_string()));
            parent.spawn(widget::small_label("[b]Damage Taken:"));
            parent.spawn(widget::small_label(stats.damage_taken.to_string()));
            parent.spawn(widget::small_label("[b]Highest Damage:"));
            parent.spawn(widget::small_label(stats.highest_damage.to_string()));
            // parent.spawn(widget::small_label("[b]Highest Flux:"));
            // parent.spawn(widget::small_label(stats.highest_flux.to_string()));
        })),
    )
}
