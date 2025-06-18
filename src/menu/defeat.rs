use crate::level::Level;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;
use crate::screen::title::TitleAssets;
use crate::stats::Stats;
use crate::stats::stats_grid;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Defeat.on_enter(spawn_defeat_menu));
}

fn spawn_defeat_menu(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    stats: Res<Stats>,
    level: NextRef<Level>,
) {
    let level = r!(level.get()).0;

    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::header("[b]Defeat"),
            stats_grid(&stats, level),
            widget::row_of_buttons(children![
                widget::small_button("Retry star", restart_level),
                widget::small_button("New mission", restart_game),
                widget::small_button("Go home", quit_to_title),
            ]),
        ]));
}

fn restart_level(trigger: Trigger<Pointer<Click>>, mut level: FlushMut<Level>) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    level.refresh();
}

fn restart_game(trigger: Trigger<Pointer<Click>>, mut level: NextMut<Level>) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    level.enter(Level(0));
    level.trigger();
}

fn quit_to_title(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    title_assets: Res<TitleAssets>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    commands.spawn(fade_out(&title_assets, Screen::Title));
}
