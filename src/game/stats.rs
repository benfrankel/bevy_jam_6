use crate::game::level::Level;
use crate::game::phase::Phase;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Stats>();
}

#[derive(Resource, Reflect, Debug, Default, Clone)]
#[reflect(Resource)]
pub struct Stats {
    pub missiles: usize,
    pub lasers: usize,
    pub fireballs: usize,
    pub repairs: usize,
    pub highest_flux: f32,
    pub damage_given: f32,
    pub damage_taken: f32,
    pub highest_damage: f32,
    pub total_rounds: usize,
}

impl Configure for Stats {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(
            StateFlush,
            (
                Level(0).on_enter(reset_stats),
                Phase::Player.on_enter(increment_total_rounds),
            ),
        );
    }
}

fn reset_stats(mut stats: ResMut<Stats>) {
    *stats = default();
}

fn increment_total_rounds(mut stats: ResMut<Stats>) {
    stats.total_rounds += 1;
}

pub fn stats_grid(stats: &Stats, level: usize) -> impl Bundle {
    let stats = stats.clone();
    (
        Name::new("StatsGrid"),
        Node {
            column_gap: Vw(4.0),
            ..Node::ROW_CENTER
        },
        children![
            (
                Name::new("LeftGrid"),
                Node {
                    display: Display::Grid,
                    justify_content: JustifyContent::End,
                    margin: UiRect::top(Vw(2.0)).with_bottom(Vw(5.2)),
                    row_gap: Vw(1.0),
                    column_gap: Vw(2.0),
                    grid_template_columns: vec![
                        RepeatedGridTrack::flex(1, 1.0),
                        RepeatedGridTrack::auto(1),
                    ],
                    ..Node::DEFAULT.grow()
                },
                GridAlignment::columns([JustifySelf::End, JustifySelf::Start]),
                Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                    parent.spawn(widget::small_label(level.to_string()));
                    parent.spawn(widget::small_label(format!(
                        "[b]level{} completed",
                        plural(level),
                    )));
                    parent.spawn(widget::small_label(stats.damage_given.to_string()));
                    parent.spawn(widget::small_label("[b]damage given"));
                    parent.spawn(widget::small_label(stats.damage_taken.to_string()));
                    parent.spawn(widget::small_label("[b]damage taken"));
                    parent.spawn(widget::small_label(stats.highest_damage.to_string()));
                    parent.spawn(widget::small_label("[b]max damage"));
                })),
            ),
            (
                Name::new("RightGrid"),
                Node {
                    display: Display::Grid,
                    justify_content: JustifyContent::Start,
                    margin: UiRect::top(Vw(2.0)).with_bottom(Vw(5.2)),
                    row_gap: Vw(1.0),
                    column_gap: Vw(2.0),
                    grid_template_columns: vec![
                        RepeatedGridTrack::auto(1),
                        RepeatedGridTrack::flex(1, 1.0),
                    ],
                    ..Node::DEFAULT.grow()
                },
                GridAlignment::columns([JustifySelf::End, JustifySelf::Start]),
                Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                    parent.spawn(widget::small_label(stats.missiles.to_string()));
                    parent.spawn(widget::small_label(format!(
                        "[b]missile{} launched",
                        plural(stats.missiles),
                    )));
                    parent.spawn(widget::small_label(stats.lasers.to_string()));
                    parent.spawn(widget::small_label(format!(
                        "[b]laser{} fired",
                        plural(stats.lasers),
                    )));
                    parent.spawn(widget::small_label(stats.fireballs.to_string()));
                    parent.spawn(widget::small_label(format!(
                        "[b]fireball{} unleashed",
                        plural(stats.fireballs),
                    )));
                    parent.spawn(widget::small_label(stats.repairs.to_string()));
                    parent.spawn(widget::small_label(format!(
                        "[b]repair{}",
                        plural(stats.repairs),
                    )));
                })),
            ),
        ],
    )
}

fn plural(n: impl ToString) -> &'static str {
    if n.to_string() == "1" { "" } else { "s" }
}
