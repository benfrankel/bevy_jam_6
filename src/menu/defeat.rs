use crate::game::level::Level;
use crate::game::stats::Stats;
use crate::game::stats::stats_grid;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;
use crate::screen::title::TitleAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Defeat.on_enter(spawn_defeat_menu));
}

fn spawn_defeat_menu(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    stats: Res<Stats>,
    level: NextRef<Level>,
) {
    let level = r!(level.get()).0.to_string();

    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::header("[b]Defeat"),
            stats_grid(stats, level),
            (
                Node::COLUMN_CENTER,
                children![widget::row_of_buttons(children![
                    widget::wide_button("Retry star", restart_level),
                    widget::wide_button("End pursuit", quit_to_title),
                ])],
            )
        ]));
}

fn restart_level(_: Trigger<Pointer<Click>>, mut level: FlushMut<Level>) {
    level.refresh();
}

fn quit_to_title(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    title_assets: Res<TitleAssets>,
) {
    commands.spawn(fade_out(&title_assets, Screen::Title));
}
