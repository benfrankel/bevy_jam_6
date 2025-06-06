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

    player_deck.max_health += level_setup.reward_max_health;
    player_deck.heat_capacity += level_setup.reward_heat_capacity;
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

    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::header("[b]They got away!"),
            (
                Node {
                    margin: UiRect::vertical(Vh(3.5)),
                    ..Node::DEFAULT
                },
                widget::small_label(format!(
                    "Reactor upgrade: +{} slots",
                    level_setup.reward_reactor_slots,
                )),
            ),
            (
                Node {
                    margin: UiRect::vertical(Vh(3.5)),
                    ..Node::DEFAULT
                },
                widget::small_label(format!(
                    "Ship upgrade: +{} hull",
                    level_setup.reward_max_health,
                )),
            ),
            (
                Node::COLUMN_CENTER.grow(),
                children![widget::row_of_buttons(children![widget::button(
                    "Next star",
                    enter_next_level,
                )])]
            ),
        ]));
}

fn enter_next_level(_: Trigger<Pointer<Click>>, mut level: NextMut<Level>) {
    r!(level.get_mut()).0 += 1;
}
