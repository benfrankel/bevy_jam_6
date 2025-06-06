use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Credits.on_enter(spawn_credits_menu));
}

fn spawn_credits_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::header("[b]Credits"),
            grid(),
            widget::row_of_buttons(children![widget::wide_button("Back", go_back)]),
        ]));
}

fn go_back(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.pop();
}

fn grid() -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            margin: UiRect::vertical(Vw(4.0)),
            row_gap: Vw(1.4),
            column_gap: Vw(6.0),
            grid_template_columns: vec![
                RepeatedGridTrack::flex(1, 1.0),
                RepeatedGridTrack::flex(1, 1.0),
            ],
            ..default()
        },
        GridAlignment::columns([JustifySelf::End, JustifySelf::Start]),
        children![
            widget::label("[b]Pyrious"),
            widget::label("Lead"),
            widget::label("[b]Median"),
            widget::label("Music and SFX"),
            widget::label("[b]Jayclees"),
            widget::label("Developer"),
        ],
    )
}
