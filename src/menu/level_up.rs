use crate::game::deck::PlayerDeck;
use crate::game::level::Level;
use crate::game::level::LevelConfig;
use crate::game::module::Module;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        Menu::LevelUp.on_enter((apply_level_up_rewards, spawn_level_up_menu)),
    );
}

fn apply_level_up_rewards(
    level_config: ConfigRef<LevelConfig>,
    level: CurrentRef<Level>,
    mut player_deck: ResMut<PlayerDeck>,
) {
    let level = r!(level.get()).0;
    let level_config = r!(level_config.get());
    let level_setup = r!(level_config.levels.get(level));
    player_deck
        .reactor
        .extend(vec![Module::EMPTY; level_setup.reward_reactor_slots]);
}

fn spawn_level_up_menu(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    level_config: ConfigRef<LevelConfig>,
    level: CurrentRef<Level>,
) {
    let level = r!(level.get()).0;
    let level_config = r!(level_config.get());
    let level_setup = r!(level_config.levels.get(level));
    commands.entity(menu_root.ui).with_child((
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Node::DEFAULT.full_size()
        },
        BackgroundColor::from(Color::srgba(0., 0., 0., 0.5)),
        Transform::default(),
        children![(
            Node {
                width: Vw(50.),
                height: Vh(90.),
                padding: UiRect::axes(Vw(5.), Vh(5.)),
                flex_direction: FlexDirection::Column,
                ..Node::DEFAULT
            },
            BackgroundColor::from(Color::srgba(0.1, 0.1, 0.1, 0.8)),
            children![
                widget::label_base(Vw(3.5), ThemeColor::White, "[b]The enemy ship escaped..."),
                (
                    Node {
                        margin: UiRect::axes(Vw(0.), Vh(3.5)),
                        ..Node::DEFAULT
                    },
                    widget::small_label(format!(
                        "Reactor upgraded: +{} slots",
                        level_setup.reward_reactor_slots,
                    )),
                ),
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        flex_grow: 1.,
                        ..Node::DEFAULT
                    },
                    children![widget::row_of_buttons(children![widget::button(
                        "Keep up the pursuit",
                        enter_next_level,
                    )]),]
                ),
            ],
        )],
    ));
}

fn enter_next_level(_: Trigger<Pointer<Click>>, mut level: NextMut<Level>) {
    r!(level.get_mut()).0 += 1;
}
