use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;
use crate::screen::title::TitleAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Victory.on_enter(spawn_victory_menu));
}

fn spawn_victory_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::header("[b]Victory"),
            (
                Node::COLUMN_CENTER.grow(),
                children![widget::column_of_buttons(children![
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
