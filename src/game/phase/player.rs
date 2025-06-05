use crate::game::deck::PlayerDeck;
use crate::game::phase::Phase;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        Phase::Player.on_update(skip_player_phase.run_if(player_has_nothing_to_do)),
    );

    app.configure::<PlayerActions>();
}

fn skip_player_phase(mut next_phase: NextMut<Phase>) {
    next_phase.enter(Phase::Reactor);
}

fn player_has_nothing_to_do(player_deck: Res<PlayerDeck>) -> bool {
    player_deck.hand.is_empty() && player_deck.storage.is_empty()
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PlayerActions {
    SelectLeft,
    SelectRight,
    PlayModule,
    EndTurn,
}

impl Configure for PlayerActions {
    fn configure(app: &mut App) {
        let mut action_state = ActionState::<Self>::default();
        action_state.disable_action(&Self::PlayModule);
        action_state.disable_action(&Self::EndTurn);
        app.insert_resource(action_state);

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
                .with(Self::PlayModule, KeyCode::NumpadEnter)
                .with(Self::EndTurn, KeyCode::KeyE),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            StateFlush,
            (
                Phase::Player.on_edge(disable_end_turn, enable_end_turn),
                Pause
                    .on_edge(enable_player_actions, disable_player_actions)
                    .run_if(Phase::Player.will_update()),
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
                player_end_turn
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::EndTurn)),
            ),
        );
    }
}

fn disable_end_turn(mut player_actions: ResMut<ActionState<PlayerActions>>) {
    player_actions.disable_action(&PlayerActions::PlayModule);
    player_actions.disable_action(&PlayerActions::EndTurn);
}

fn enable_end_turn(mut player_actions: ResMut<ActionState<PlayerActions>>) {
    player_actions.enable_action(&PlayerActions::PlayModule);
    player_actions.enable_action(&PlayerActions::EndTurn);
}

fn disable_player_actions(mut player_actions: ResMut<ActionState<PlayerActions>>) {
    info!("PLAYER ACTIONS: Disable");
    player_actions.disable();
}

fn enable_player_actions(mut player_actions: ResMut<ActionState<PlayerActions>>) {
    info!("PLAYER ACTIONS: Enable");
    player_actions.enable();
}

fn player_select_left(mut player_deck: ResMut<PlayerDeck>) {
    player_deck.bypass_change_detection().advance_selected(-1);
}

fn player_select_right(mut player_deck: ResMut<PlayerDeck>) {
    player_deck.bypass_change_detection().advance_selected(1);
}

fn player_play_module(mut player_deck: ResMut<PlayerDeck>, mut next_phase: NextMut<Phase>) {
    if player_deck.play_selected() {
        next_phase.enter(Phase::Reactor);
    }
}

fn player_end_turn(mut next_phase: NextMut<Phase>) {
    next_phase.enter(Phase::Reactor);
}
