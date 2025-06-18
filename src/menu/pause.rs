use crate::level::Level;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;
use crate::screen::title::TitleAssets;

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
                widget::wide_button("Settings", open_settings),
                widget::wide_button("Retry star", restart_level),
                widget::wide_button("End pursuit", quit_to_title),
            ])
        ]));
}

fn close_menu(trigger: Trigger<Pointer<Click>>, mut menu: NextMut<Menu>) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    menu.disable();
}

fn restart_level(trigger: Trigger<Pointer<Click>>, mut level: FlushMut<Level>) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    level.refresh();
}

fn open_settings(trigger: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    menu.push(Menu::Settings);
}

fn quit_to_title(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    title_assets: Res<TitleAssets>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    commands.spawn(fade_out(&title_assets, Screen::Title));
}
