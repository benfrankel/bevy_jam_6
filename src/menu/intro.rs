use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;
use crate::screen::title::TitleAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Intro.on_enter(spawn_intro_menu));
}

fn spawn_intro_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::header("[b]Mission received:"),
            (
                widget::label_base(
                    Vw(3.5),
                    ThemeColor::BodyText,
                    JustifyText::Center,
                    1.6,
                    "Protect our home star\n\
                    against the tyrant threat from afar.\n\
                    Command The Weber?"
                ),
                Node {
                    margin: UiRect::top(Vw(-1.0)).with_bottom(Vw(4.0)),
                    ..default()
                },
            ),
            widget::row_of_buttons(children![
                widget::button("Decline", go_back),
                widget::button("Pursue", start_game)
            ]),
        ]));
}

fn go_back(trigger: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    menu.pop();
}

fn start_game(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    title_assets: Res<TitleAssets>,
    progress: Res<ProgressTracker<BevyState<Screen>>>,
    mut menu: ResMut<NextStateStack<Menu>>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    let Progress { done, total } = progress.get_global_combined_progress();
    if done >= total {
        commands.spawn(fade_out(&title_assets, Screen::Gameplay));
    } else {
        menu.push(Menu::Loading);
    }
}
