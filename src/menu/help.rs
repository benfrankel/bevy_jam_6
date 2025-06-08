use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Help.on_enter(spawn_help_menu));
}

fn spawn_help_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::label_base(
                Vw(1.8),
                ThemeColor::BodyText,
                JustifyText::Left,
                1.8,
                "Welcome aboard [b]The Weber[r]!\
                \n\n\
                Insert modules into the [b]reactor[r] to forge powerful chain reactions. \
                Every module has a [b]condition[r] and an [b]effect[r]. \
                The reactor will always activate the first matching module from the top left.\
                \n\n\
                When a module activates, the reactor's [b]flux[r] increases by 1, which adds to \
                the module's [b]heat[r]. Overheated modules must be replaced.\
                \n\n"
            ),
            widget::row_of_buttons(children![widget::button("Close manual", go_back)]),
        ]));
}

fn go_back(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.pop();
}
