use crate::game::level::Level;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::title::TitleAssets;
use crate::screen::Screen;
use crate::screen::fade::fade_out;

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
                widget::wide_button("Restart level", restart_level),
                widget::wide_button("Settings", open_settings),
                widget::wide_button("End run", quit_to_title),
            ])
        ]));
}

fn close_menu(_: Trigger<Pointer<Click>>, mut menu: NextMut<Menu>) {
    menu.disable();
}

fn restart_level(_: Trigger<Pointer<Click>>, mut level: FlushMut<Level>) {
    level.refresh();
}

fn open_settings(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Settings);
}

fn quit_to_title(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    title_assets: Res<TitleAssets>,
) {
    commands.spawn(fade_out(&title_assets, Screen::Title));
}
