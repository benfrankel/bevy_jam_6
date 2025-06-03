use crate::game::deck::PlayerDeck;
use crate::game::turn::Turn;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<PlayerActions>();
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum PlayerActions {
    SelectLeft,
    SelectRight,
    PlayModule,
}

impl Configure for PlayerActions {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::SelectLeft, GamepadButton::DPadLeft)
                .with(Self::SelectLeft, GamepadButton::LeftTrigger)
                .with(Self::SelectLeft, KeyCode::KeyA)
                .with(Self::SelectLeft, KeyCode::ArrowLeft)
                .with(Self::SelectRight, GamepadButton::DPadRight)
                .with(Self::SelectRight, GamepadButton::RightTrigger)
                .with(Self::SelectRight, KeyCode::KeyD)
                .with(Self::SelectRight, KeyCode::ArrowRight)
                .with(Self::PlayModule, GamepadButton::East)
                .with(Self::PlayModule, KeyCode::Space)
                .with(Self::PlayModule, KeyCode::Enter)
                .with(Self::PlayModule, KeyCode::NumpadEnter),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            StateFlush,
            (
                Turn::Player.on_edge(disable_play_module, enable_play_module),
                Pause
                    .on_edge(enable_player_actions, disable_player_actions)
                    .run_if(Turn::Player.will_update()),
            ),
        );
        app.add_systems(
            Update,
            (
                player_select_left
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::SelectLeft)),
                player_select_right
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::SelectRight)),
                player_play_module
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::PlayModule)),
            ),
        );
    }
}

fn disable_play_module(mut player_actions: ResMut<ActionState<PlayerActions>>) {
    player_actions.disable_action(&PlayerActions::PlayModule);
}

fn enable_play_module(mut player_actions: ResMut<ActionState<PlayerActions>>) {
    player_actions.enable_action(&PlayerActions::PlayModule);
}

fn disable_player_actions(mut player_actions: ResMut<ActionState<PlayerActions>>) {
    player_actions.disable();
}

fn enable_player_actions(mut player_actions: ResMut<ActionState<PlayerActions>>) {
    player_actions.enable();
}

fn player_select_left(mut player_deck: ResMut<PlayerDeck>) {
    player_deck.advance_selected(-1);
}

fn player_select_right(mut player_deck: ResMut<PlayerDeck>) {
    player_deck.advance_selected(1);
}

fn player_play_module(mut player_deck: ResMut<PlayerDeck>, mut next_turn: NextMut<Turn>) {
    player_deck.play_selected();
    next_turn.enter(Turn::Reactor);
}
