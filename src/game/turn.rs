use crate::game::GameLayer;
use crate::game::deck::Deck;
use crate::game::level::Level;
use crate::game::missile::MissileAssets;
use crate::game::missile::missile;
use crate::game::ship::IsEnemyShip;
use crate::game::ship::IsWeapon;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Turn, PlayerActions)>();
}

#[derive(State, Reflect, Copy, Clone, Default, Eq, PartialEq, Debug)]
#[state(log_flush, after(Level))]
#[reflect(Resource)]
pub enum Turn {
    #[default]
    Player,
    Reactor,
    Enemy,
}

impl Configure for Turn {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(
            StateFlush,
            (
                Level::ANY.on_edge(Turn::disable, (Turn::enable_default, Turn::trigger)),
                Turn::Enemy.on_enter(fire_enemy_missile),
            ),
        );
        app.add_systems(
            Update,
            (
                // TODO: Implement reactor turn.
                Turn::Reactor.on_update(Turn::Enemy.enter()),
                // TODO: Implement enemy turn.
                Turn::Enemy.on_update(Turn::Player.enter()),
            ),
        );
    }
}

fn fire_enemy_missile(
    mut commands: Commands,
    missile_assets: Res<MissileAssets>,
    enemy_ship_children: Single<&Children, With<IsEnemyShip>>,
    weapon_query: Query<&GlobalTransform, With<IsWeapon>>,
) {
    let weapons = enemy_ship_children
        .iter()
        .filter_map(|entity| weapon_query.get(entity).ok())
        .collect::<Vec<_>>();
    let gt = **r!(weapons.choose(&mut thread_rng()));
    commands.spawn((
        missile(&missile_assets, thread_rng().gen_range(0.0..15.0)),
        CollisionLayers::new(GameLayer::Default, GameLayer::Player),
        gt.compute_transform(),
        gt,
        DespawnOnExitState::<Level>::default(),
    ));
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
            Turn::Player.on_edge(disable_player_actions, enable_player_actions),
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

fn disable_player_actions(mut player_actions: ResMut<ActionState<PlayerActions>>) {
    player_actions.disable();
}

fn enable_player_actions(mut player_actions: ResMut<ActionState<PlayerActions>>) {
    player_actions.enable();
}

fn player_select_left(mut deck: ResMut<Deck>) {
    deck.advance(-1);
}

fn player_select_right(mut deck: ResMut<Deck>) {
    deck.advance(1);
}

fn player_play_module(mut deck: ResMut<Deck>, mut next_turn: NextMut<Turn>) {
    deck.play();
    next_turn.enter(Turn::Reactor);
}
