use crate::game::level::LevelAssets;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::theme::color::ThemeConfig;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Main.on_enter(spawn_main_menu));
}

fn spawn_main_menu(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    level_assets: Res<LevelAssets>,
    theme_config: ConfigRef<ThemeConfig>,
) {
    let theme_config = r!(theme_config.get());
    commands.spawn((
        Name::new("Background"),
        Sprite::from_image(level_assets.bg_level0.clone()),
        Transform::from_xyz(0.0, 0.0, -2.0),
        DespawnOnExitState::<Menu>::Recursive,
    ));
    commands.entity(menu_root.ui).with_child((
        Name::new("MainMenuContainer"),
        Node {
            ..Node::DEFAULT.full_size()
        },
        children![
            (
                Name::new("ButtonsContainer"),
                Node {
                    width: Vw(1. / 3. * 100.),
                    justify_content: JustifyContent::Center,
                    border: UiRect::right(Px(1.5)),
                    ..Node::DEFAULT
                },
                BackgroundColor::from(theme_config.colors[ThemeColor::Popup]),
                BorderColor::from(theme_config.colors[ThemeColor::BorderColor]),
                BoxShadow::from(ShadowStyle {
                    y_offset: Vh(0.),
                    ..ShadowStyle::default()
                }),
                children![widget::column_of_buttons(children![
                    widget::button("Play", open_intro),
                    widget::button("Settings", open_settings),
                    (
                        widget::button("Quit", quit_to_desktop),
                        #[cfg(feature = "web")]
                        InteractionDisabled(true),
                    ),
                ]),]
            ),
            (
                Name::new("TitleContainer"),
                Node {
                    width: Vw(2. / 3. * 100.),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::top(Vh(15.)),
                    ..Node::DEFAULT
                },
                BackgroundColor::from(theme_config.colors[ThemeColor::Overlay]),
                children![widget::header("[b]Bevy Jam 6"),],
            ),
        ],
    ));
    // .with_child(widget::body(children![
    //     widget::header("[b]Bevy Jam 6"),
    //     widget::column_of_buttons(children![
    //         widget::big_button("Play", open_intro),
    //         widget::big_button("Settings", open_settings),
    //         (
    //             widget::big_button("Quit", quit_to_desktop),
    //             #[cfg(feature = "web")]
    //             InteractionDisabled(true),
    //         ),
    //     ]),
    // ]));
}

fn open_intro(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Intro);
}

fn open_settings(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Settings);
}

fn quit_to_desktop(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    if cfg!(not(feature = "web")) {
        app_exit.write(AppExit::Success);
    }
}
