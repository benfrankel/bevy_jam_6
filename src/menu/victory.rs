use crate::level::Level;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;
use crate::screen::title::TitleAssets;
use crate::stats::Stats;
use crate::stats::stats_grid;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Victory.on_enter(spawn_victory_menu));
}

fn spawn_victory_menu(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    stats: Res<Stats>,
    level: NextRef<Level>,
) {
    let level = r!(level.get()).0 + 1;

    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::header("[b]Victory"),
            stats_grid(&stats, level),
            widget::row_of_buttons(children![
                widget::wide_button("New mission", restart_game),
                widget::wide_button("Go home", quit_to_title),
            ]),
        ]));
}

fn restart_game(_: Trigger<Pointer<Click>>, mut level: NextMut<Level>) {
    level.enter(Level(0));
    level.trigger();
}

fn quit_to_title(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    title_assets: Res<TitleAssets>,
) {
    commands.spawn(fade_out(&title_assets, Screen::Title));
}
