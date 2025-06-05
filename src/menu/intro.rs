use crate::game::level::LevelAssets;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;
use crate::theme::color::ThemeConfig;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Intro.on_enter(spawn_intro_menu));
}

fn spawn_intro_menu(
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
    commands.entity(menu_root.ui).with_child(((
        Node::COLUMN_CENTER.full_size(),
        BackgroundColor::from(theme_config.colors[ThemeColor::Overlay]),
        children![
            widget::header("[b]How to play"),
            widget::paragraph("Be skillful,\nwin the game!\nPress P to pause."),
            widget::row_of_buttons(children![
                widget::button("Back", go_back),
                widget::button("Start", start_game)
            ]),
        ],
    ),));
}

fn go_back(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.pop();
}

fn start_game(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    progress: Res<ProgressTracker<BevyState<Screen>>>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();
    commands.spawn(fade_out(if done >= total {
        Screen::Gameplay
    } else {
        Screen::Loading
    }));
}
