use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;
use crate::theme::color::ThemeConfig;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Defeat.on_enter(spawn_defeat_menu));
}

fn spawn_defeat_menu(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    theme_config: ConfigRef<ThemeConfig>,
) {
    let theme_config = r!(theme_config.get());
    commands.entity(menu_root.ui).with_child((
        Name::new("DefeatPopup"),
        widget::overlay(theme_config),
        children![(
            widget::popup_window(Vw(50.), Vh(90.), theme_config, None, None),
            children![
                widget::big_label("Defeat"),
                (
                    Node {
                        flex_grow: 1.,
                        ..Node::COLUMN_CENTER
                    },
                    children![widget::column_of_buttons(children![
                        widget::wide_button("Try again", restart_game),
                        widget::wide_button("Main menu", quit_to_title),
                    ])],
                )
            ],
        )],
    ));
}

fn restart_game(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.spawn(fade_out(Screen::Gameplay));
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.spawn(fade_out(Screen::Title));
}
