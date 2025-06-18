use crate::deck::PlayerDeck;
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
            widget::paragraph(format!(
                "Welcome aboard [b]{}[r]!\n\
                \n\
                [b]The Helm[r] (bottom)\n\
                - Left click to insert a module into the reactor.\n\
                - Right click to remove a module from the reactor or helm.\n\
                - Press Space to end your turn.\n\
                \n\
                [b]The Reactor[r] (left)\n\
                - Each module has a [b]Condition[r] -> [b]Effect[r].\n\
                - The reactor always activates the first matching module.\n\
                - Modules gain heat equal to the length of the current chain.\n\
                - Flux tracks the longest chain and boosts your power.\n\
                \n",
                player_deck.name,
            ),),
            widget::row_of_buttons(children![widget::small_button("Close manual", go_back)]),
        ]));
}

fn go_back(trigger: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    menu.pop();
}
