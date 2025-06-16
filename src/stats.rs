use crate::level::Level;
use crate::phase::Phase;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Stats>();
}

#[derive(Resource, Reflect, Debug, Default, Clone)]
#[reflect(Resource)]
pub struct Stats {
    pub actions: HashMap<String, usize>,
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
    let repairs = stats.actions.get("repair").copied().unwrap_or_default();
    let missiles = stats.actions.get("missile").copied().unwrap_or_default();
    let lasers = stats.actions.get("laser").copied().unwrap_or_default();
    let fireballs = stats.actions.get("fireball").copied().unwrap_or_default();

    (
        Name::new("StatsGrid"),
        Node {
            column_gap: Vw(4.0),
            ..Node::ROW.center()
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
                children![
                    widget::small_label(level.to_string()),
                    widget::small_label(format!("[b]star{} defended", plural(level))),
                    widget::small_label(stats.damage_given.to_string()),
                    widget::small_label("[b]damage given"),
                    widget::small_label(stats.damage_taken.to_string()),
                    widget::small_label("[b]damage taken"),
                    widget::small_label(stats.highest_damage.to_string()),
                    widget::small_label("[b]max damage"),
                ],
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
                    parent.spawn(widget::small_label(missiles.to_string()));
                    parent.spawn(widget::small_label(format!(
                        "[b]missile{} launched",
                        plural(missiles),
                    )));
                    parent.spawn(widget::small_label(lasers.to_string()));
                    parent.spawn(widget::small_label(format!(
                        "[b]laser{} fired",
                        plural(lasers),
                    )));
                    parent.spawn(widget::small_label(fireballs.to_string()));
                    parent.spawn(widget::small_label(format!(
                        "[b]fireball{} unleashed",
                        plural(fireballs),
                    )));
                    parent.spawn(widget::small_label(repairs.to_string()));
                    parent.spawn(widget::small_label(format!("[b]repair{}", plural(repairs))));
                })),
            ),
        ],
    )
}

fn plural(n: impl ToString) -> &'static str {
    if n.to_string() == "1" { "" } else { "s" }
}
