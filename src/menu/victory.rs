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
    app.add_systems(StateFlush, Menu::Victory.on_enter(spawn_victory_menu));
}

fn spawn_victory_menu(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    stats: Res<Stats>,
    level: NextRef<Level>,
) {
    let level = (r!(level.get()).0 + 1).to_string();

    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::header("[b]Victory"),
            stats_grid(stats, level),
            (
                Node::ROW_CENTER.grow(),
                children![widget::row_of_buttons(children![
                    widget::wide_button("Play Again", restart_game),
                    widget::wide_button("Main menu", quit_to_title),
                ])],
            )
        ]));
}

fn restart_game(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    title_assets: Res<TitleAssets>,
) {
    commands.spawn(fade_out(&title_assets, Screen::Gameplay));
}

fn quit_to_title(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    title_assets: Res<TitleAssets>,
) {
    commands.spawn(fade_out(&title_assets, Screen::Title));
}
