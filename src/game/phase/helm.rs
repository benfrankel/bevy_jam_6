use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::game::GameAssets;
use crate::game::deck::PlayerDeck;
use crate::game::level::Level;
use crate::game::phase::Phase;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        Phase::Helm.on_update(skip_helm_phase.run_if(helm_is_empty)),
    );

    app.configure::<HelmActions>();
}

fn skip_helm_phase(mut phase: NextMut<Phase>) {
    phase.enter(Phase::Reactor);
}

fn helm_is_empty(player_deck: Res<PlayerDeck>) -> bool {
    player_deck.hand.is_empty() && player_deck.storage.is_empty()
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum HelmActions {
    SelectLeft,
    SelectRight,
    PlayModule,
    DiscardModule,
    EndTurn,
}

impl Configure for HelmActions {
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
                .with(Self::PlayModule, KeyCode::Space)
                .with(Self::PlayModule, KeyCode::Enter)
                .with(Self::PlayModule, KeyCode::NumpadEnter)
                .with(Self::DiscardModule, KeyCode::Delete)
                .with(Self::DiscardModule, KeyCode::Backspace)
                .with(Self::DiscardModule, KeyCode::NumpadBackspace)
                .with(Self::EndTurn, KeyCode::KeyE),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(Startup, disable_helm_phase_actions);
        app.add_systems(
            StateFlush,
            (
                Phase::Helm.on_edge(disable_helm_phase_actions, enable_helm_phase_actions),
                Pause.on_edge(enable_helm_actions, disable_helm_actions),
            ),
        );
        app.add_systems(
            Update,
            (
                helm_select_left
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::SelectLeft)),
                helm_select_right
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::SelectRight)),
                helm_play_module
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::PlayModule)),
                helm_discard_module
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::DiscardModule)),
                helm_end_turn
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::EndTurn)),
            ),
        );
    }
}

fn disable_helm_phase_actions(mut helm_actions: ResMut<ActionState<HelmActions>>) {
    helm_actions.disable_action(&HelmActions::PlayModule);
    helm_actions.disable_action(&HelmActions::DiscardModule);
    helm_actions.disable_action(&HelmActions::EndTurn);
}

fn enable_helm_phase_actions(mut helm_actions: ResMut<ActionState<HelmActions>>) {
    helm_actions.enable_action(&HelmActions::PlayModule);
    helm_actions.enable_action(&HelmActions::DiscardModule);
    helm_actions.enable_action(&HelmActions::EndTurn);
}

fn disable_helm_actions(mut helm_actions: ResMut<ActionState<HelmActions>>) {
    helm_actions.disable();
}

fn enable_helm_actions(mut helm_actions: ResMut<ActionState<HelmActions>>) {
    helm_actions.enable();
}

fn helm_select_left(
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    game_assets: Res<GameAssets>,
    mut player_deck: ResMut<PlayerDeck>,
) {
    player_deck.bypass_change_detection().advance_selected(-1);
    commands.spawn((
        sfx_audio(&audio_settings, game_assets.module_hover_sfx.clone(), 1.0),
        DespawnOnExitState::<Level>::default(),
    ));
}

fn helm_select_right(
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    game_assets: Res<GameAssets>,
    mut player_deck: ResMut<PlayerDeck>,
) {
    player_deck.bypass_change_detection().advance_selected(1);
    commands.spawn((
        sfx_audio(&audio_settings, game_assets.module_hover_sfx.clone(), 1.0),
        DespawnOnExitState::<Level>::default(),
    ));
}

fn helm_play_module(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    audio_settings: Res<AudioSettings>,
    mut player_deck: ResMut<PlayerDeck>,
    mut phase: NextMut<Phase>,
) {
    rq!(player_deck.play_selected(&mut thread_rng()));

    phase.enter(Phase::Reactor);
    commands.spawn((
        sfx_audio(&audio_settings, game_assets.module_insert_sfx.clone(), 1.0),
        DespawnOnExitState::<Level>::default(),
    ));
}

fn helm_discard_module(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    audio_settings: Res<AudioSettings>,
    mut player_deck: ResMut<PlayerDeck>,
    mut phase: NextMut<Phase>,
) {
    rq!(player_deck.discard_selected(&mut thread_rng()));

    commands.spawn((
        sfx_audio(&audio_settings, game_assets.module_hover_sfx.clone(), 1.0),
        DespawnOnExitState::<Level>::default(),
    ));
    if player_deck.hand.is_empty() {
        phase.enter(Phase::Reactor);
    }
}

fn helm_end_turn(mut phase: NextMut<Phase>) {
    phase.enter(Phase::Reactor);
}
