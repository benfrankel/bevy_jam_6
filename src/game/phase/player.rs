use crate::game::deck::HandIndex;
use crate::game::deck::PlayerDeck;
use crate::game::phase::Phase;
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
        let mut action_state = ActionState::<Self>::default();
        action_state.disable_action(&Self::PlayModule);
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
                .with(Self::PlayModule, KeyCode::NumpadEnter),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            StateFlush,
            (
                Phase::Player.on_edge(disable_play_module, enable_play_module),
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
                player_play_module_on_click.in_set(UpdateSystems::Update),
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

fn player_play_module(mut player_deck: ResMut<PlayerDeck>, mut next_phase: NextMut<Phase>) {
    if player_deck.play_selected() {
        next_phase.enter(Phase::Reactor);
    }
}

fn player_play_module_on_click(
    interaction_query: Query<&Interaction, With<HandIndex>>,
    mut player_actions: ResMut<ActionState<PlayerActions>>,
) {
    if interaction_query.iter().any(|&x| x == Interaction::Pressed) {
        player_actions.press(&PlayerActions::PlayModule);
    }
}
