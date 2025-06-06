use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Credits.on_enter(spawn_credits_menu));
}

fn spawn_credits_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands.entity(menu_root.ui).with_child(widget::popup(
        Vw(70.0),
        Vh(90.0),
        children![
            widget::header("[b]Credits"),
            widget::label("Pyrious (Lead)"),
            spacer(),
            widget::label("Median (Music and SFX)"),
            spacer(),
            widget::label("Jayclees (Developer)"),
            spacer(),
            (
                Node::ROW_CENTER,
                children![widget::wide_button("Back", go_back)]
            ),
        ],
    ));
}

fn go_back(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.pop();
}

fn spacer() -> impl Bundle {
    Node {
        margin: UiRect::vertical(Vw(2.5)),
        ..Node::DEFAULT
    }
}
