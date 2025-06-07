use crate::game::deck::PlayerDeck;
use crate::game::level::Level;
use crate::game::level::LevelConfig;
use crate::game::level::LevelReward;
use crate::game::module::Module;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(SelectedRewards, EnableContinueButton)>();
    app.register_type::<IsNextLevelButton>();
    app.add_systems(StateFlush, Menu::LevelUp.on_enter(spawn_level_up_menu));
}

#[derive(Resource, Reflect, Default, Debug, Clone)]
#[reflect(Resource)]
struct SelectedRewards {
    selected: Vec<LevelReward>,
}

impl Configure for SelectedRewards {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

#[derive(Component, Reflect, Debug, Copy, Clone)]
#[reflect(Component)]
struct IsNextLevelButton;

#[derive(Event, Reflect, Debug, Copy, Clone)]
struct EnableContinueButton;

impl Configure for EnableContinueButton {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

fn spawn_level_up_menu(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    level_config: ConfigRef<LevelConfig>,
    level: CurrentRef<Level>,
) {
    let level = r!(level.get()).0;
    let level_config = r!(level_config.get());
    let fixed_rewards = r!(level_config.levels.get(level)).fixed_rewards.clone();

    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::header("[b]They got away!"),
            widget::row_of_buttons(Children::spawn(SpawnWith(
                move |parent: &mut ChildSpawner| {
                    for reward in fixed_rewards.iter() {
                        match reward {
                            LevelReward::MaxHealth(health) => {
                                parent.spawn(max_health_reward(*health))
                            },
                            LevelReward::HeatCapacity(heat) => {
                                parent.spawn(heat_capacity_reward(*heat))
                            },
                            LevelReward::ReactorSlots(slots) => {
                                parent.spawn(reactor_slots_reward(*slots))
                            },
                            LevelReward::Module(module) => parent.spawn(module_reward(*module)),
                        };
                    }
                }
            ))),
            (
                Node::COLUMN_CENTER.grow(),
                children![widget::row_of_buttons(children![(
                    IsNextLevelButton,
                    widget::button("Next star", enter_next_level),
                    Patch(|entity| {
                        r!(entity.get_mut::<InteractionDisabled>()).0 = true;
                        entity.observe(enable_continue_button);
                    }),
                )])]
            ),
        ]));
}

fn enter_next_level(
    trigger: Trigger<Pointer<Click>>,
    mut level: NextMut<Level>,
    buttons_query: Query<&InteractionDisabled, With<Button>>,
    mut selected_rewards: ResMut<SelectedRewards>,
) {
    let entity = r!(trigger.get_target());
    let interaction_disabled = r!(buttons_query.get(entity));

    if interaction_disabled.0 == true {
        return;
    }

    selected_rewards.selected = vec![];

    r!(level.get_mut()).0 += 1;
}

fn max_health_reward(health: f32) -> impl Bundle {
    widget::button(
        format!("Hull +{health}"),
        move |trigger: Trigger<Pointer<Click>>,
              mut commands: Commands,
              mut player_deck: ResMut<PlayerDeck>,
              mut selected_rewards: ResMut<SelectedRewards>,
              mut buttons_query: Query<&mut InteractionDisabled, With<Button>>,
              next_level_button: Single<Entity, With<IsNextLevelButton>>| {
            let mut interaction_disabled = r!(buttons_query.get_mut(r!(trigger.get_target())));

            if interaction_disabled.0 == true {
                return;
            }

            // Return if 3 rewards already selected.
            if selected_rewards.selected.iter().count() == 3 {
                return;
            }

            // Update player max health.
            player_deck.max_health += health;
            interaction_disabled.0 = true;
            selected_rewards
                .selected
                .push(LevelReward::MaxHealth(health));

            if selected_rewards.selected.iter().count() == 3 {
                commands
                    .entity(*next_level_button)
                    .trigger(EnableContinueButton);
            }
        },
    )
}

fn heat_capacity_reward(heat: f32) -> impl Bundle {
    widget::button(
        format!("Heat Cap +{heat}"),
        move |trigger: Trigger<Pointer<Click>>,
              mut commands: Commands,
              mut player_deck: ResMut<PlayerDeck>,
              mut selected_rewards: ResMut<SelectedRewards>,
              mut buttons_query: Query<&mut InteractionDisabled, With<Button>>,
              next_level_button: Single<Entity, With<IsNextLevelButton>>| {
            let mut interaction_disabled = r!(buttons_query.get_mut(r!(trigger.get_target())));

            if interaction_disabled.0 == true {
                return;
            }

            // Return if 3 rewards already selected.
            if selected_rewards.selected.iter().count() == 3 {
                return;
            }

            // Update heat capacity.
            player_deck.heat_capacity += heat;
            interaction_disabled.0 = true;
            selected_rewards
                .selected
                .push(LevelReward::HeatCapacity(heat));

            if selected_rewards.selected.iter().count() == 3 {
                commands
                    .entity(*next_level_button)
                    .trigger(EnableContinueButton);
            }
        },
    )
}

fn reactor_slots_reward(slots: usize) -> impl Bundle {
    widget::button(
        format!("Slots +{slots}"),
        move |trigger: Trigger<Pointer<Click>>,
              mut commands: Commands,
              mut player_deck: ResMut<PlayerDeck>,
              mut selected_rewards: ResMut<SelectedRewards>,
              mut buttons_query: Query<&mut InteractionDisabled, With<Button>>,
              next_level_button: Single<Entity, With<IsNextLevelButton>>| {
            let mut interaction_disabled = r!(buttons_query.get_mut(r!(trigger.get_target())));

            if interaction_disabled.0 == true {
                return;
            }

            // Return if 3 rewards already selected.
            if selected_rewards.selected.iter().count() == 3 {
                return;
            }

            // Increase slot capacity.
            player_deck.reactor.extend(vec![Module::EMPTY; slots]);
            interaction_disabled.0 = true;
            selected_rewards
                .selected
                .push(LevelReward::ReactorSlots(slots));

            if selected_rewards.selected.iter().count() == 3 {
                commands
                    .entity(*next_level_button)
                    .trigger(EnableContinueButton);
            }
        },
    )
}

fn module_reward(_module: Module) -> impl Bundle {
    widget::button(
        "Module",
        move |_: Trigger<Pointer<Click>>, _player_deck: ResMut<PlayerDeck>| {
            // TODO: Make new module available
        },
    )
}

fn enable_continue_button(
    trigger: Trigger<EnableContinueButton>,
    mut buttons_query: Query<&mut InteractionDisabled, With<Button>>,
) {
    let entity = r!(trigger.get_target());
    let mut interaction_disabled = r!(buttons_query.get_mut(entity));
    interaction_disabled.0 = false;
}
