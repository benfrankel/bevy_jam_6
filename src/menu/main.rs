use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Main.on_enter(spawn_main_menu));
}

fn spawn_main_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands.entity(menu_root.ui).with_child((
        Name::new("MainMenuContainer"),
        Node::ROW.full_size(),
        children![side_panel(), title()],
    ));
}

fn side_panel() -> impl Bundle {
    (
        Name::new("SidePanel"),
        Node {
            padding: UiRect::all(Vw(2.0)),
            border: UiRect::right(Px(1.0)),
            ..Node::COLUMN_CENTER
        },
        ThemeColor::Popup.set::<BackgroundColor>(),
        ThemeColor::BorderColor.set::<BorderColor>(),
        BoxShadow::from(ShadowStyle {
            color: Color::BLACK.with_alpha(0.7),
            x_offset: Val::ZERO,
            y_offset: Val::ZERO,
            spread_radius: Vw(8.0),
            blur_radius: Vw(2.5),
        }),
        children![widget::column_of_buttons(children![
            widget::button("Play", open_intro),
            widget::button("Settings", open_settings),
            widget::button("Credits", open_credits),
            (
                widget::button("Quit", quit_to_desktop),
                Patch(|entity| {
                    #[cfg(feature = "web")]
                    InteractionDisabled(true);
                    r!(entity.get_mut::<InteractionDisabled>()).0 = true;
                })
            ),
        ])],
    )
}

fn title() -> impl Bundle {
    (
        Name::new("Title"),
        Node {
            padding: UiRect::top(Vw(6.0)),
            ..Node::COLUMN_MID.full_size()
        },
        children![widget::header("[b]Flux Pursuit")],
    )
}

fn open_intro(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Intro);
}

fn open_settings(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Settings);
}

fn open_credits(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Credits);
}

fn quit_to_desktop(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    if cfg!(not(feature = "web")) {
        app_exit.write(AppExit::Success);
    }
}
