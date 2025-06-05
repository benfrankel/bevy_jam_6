use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::menu::quit_to_title;
use crate::menu::restart_game;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Pause.on_enter(spawn_pause_menu));
}

fn spawn_pause_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::body(children![
            widget::header("[b]Game paused"),
            widget::column_of_buttons(children![
                widget::wide_button("Continue", close_menu),
                widget::wide_button("Restart", restart_game),
                widget::wide_button("Settings", open_settings),
                widget::wide_button("Quit to title", quit_to_title),
            ])
        ]));
}

fn open_settings(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Settings);
}

fn close_menu(_: Trigger<Pointer<Click>>, mut menu: NextMut<Menu>) {
    menu.disable();
}
