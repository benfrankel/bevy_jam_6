use crate::game::deck::PlayerDeck;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Help.on_enter(spawn_help_menu));
}

fn spawn_help_menu(mut commands: Commands, menu_root: Res<MenuRoot>, player_deck: Res<PlayerDeck>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::label_base(
                Vw(1.8),
                ThemeColor::BodyText,
                JustifyText::Left,
                1.8,
                format!(
                    "> Welcome aboard [b]{}[r]!\
                    \n\n\
                    > Insert modules into the [b]reactor[r] to forge powerful chain reactions. \
                    Every module has a [b]condition[r] and an [b]effect[r]. \
                    The reactor will always activate the first matching module from the top left.\
                    \n\n\
                    > When a module activates, it gains [b]heat[r] equal to the length of the current \
                    chain, which might make it overheat! [b]Flux[r] tracks the length of the longest \
                    chain and boosts your power.\
                    \n\n",
                    player_deck.name,
                ),
            ),
            widget::row_of_buttons(children![widget::button("Close manual", go_back)]),
        ]));
}

fn go_back(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.pop();
}
