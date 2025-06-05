use crate::game::level::LevelAssets;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Credits.on_enter(spawn_credits_menu));
}

fn spawn_credits_menu(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    level_assets: Res<LevelAssets>,
) {
    commands.spawn((
        Name::new("Background"),
        Sprite::from_image(level_assets.bg_level0.clone()),
        Transform::from_xyz(0.0, 0.0, -2.0),
        DespawnOnExitState::<Menu>::Recursive,
    ));
    commands.entity(menu_root.ui).with_child((
        widget::overlay(),
        children![
            Name::new("CreditsPopup"),
            (
                widget::popup_window(Vw(70.), Vh(90.), None, None),
                children![
                    widget::header("[b]Settings"),
                    widget::label("Pyrious (Lead)"),
                    spacer(),
                    widget::label("Eden (Audio Engineer)"),
                    spacer(),
                    widget::label("Jayclees (Developer)"),
                    spacer(),
                    (
                        Node::ROW_CENTER,
                        children![widget::wide_button("Back", go_back)]
                    ),
                ],
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
