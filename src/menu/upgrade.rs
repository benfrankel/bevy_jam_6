use rand::seq::index::sample_weighted;

use crate::animation::backup::Backup;
use crate::animation::offset::NodeOffset;
use crate::game::GameAssets;
use crate::game::deck::PlayerDeck;
use crate::game::level::Level;
use crate::game::level::LevelConfig;
use crate::game::module::Module;
use crate::game::module::ModuleAction;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(UpgradeHistory, IsNextLevelButton, UpgradeSelector)>();

    app.add_systems(StateFlush, Menu::Upgrade.on_enter(spawn_upgrade_menu));
}

#[derive(Reflect, Clone, Debug)]
pub enum Upgrade {
    FluxCapacitor,
    QuantumCooler,
    AlienAlloy,
    NothingPack(Vec<Module>),
    RepairPack(Vec<Module>),
    MissilePack(Vec<Module>),
    LaserPack(Vec<Module>),
    FireballPack(Vec<Module>),
}

fn spawn_upgrade_menu(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    game_assets: Res<GameAssets>,
    level_config: ConfigRef<LevelConfig>,
    level: NextRef<Level>,
    player_deck: Res<PlayerDeck>,
    upgrade_history: Res<UpgradeHistory>,
) {
    let _level = r!(level.get()).0;
    let _level_config = r!(level_config.get());

    // Generate upgrade offers.
    let upgrades = generate_upgrades(
        &mut thread_rng(),
        player_deck.reactor.len(),
        &upgrade_history,
    );

    commands
        .entity(menu_root.ui)
        .with_child(widget::popup(children![
            widget::header("[b]They slipped away!"),
            widget::label("Choose 3 upgrades:"),
            offered_upgrades(&game_assets, upgrades),
            widget::row_of_buttons(children![(
                IsNextLevelButton,
                widget::button("Pursue", enter_next_level),
                Patch(|entity| {
                    r!(entity.get_mut::<InteractionDisabled>()).0 = true;
                }),
            )])
        ]));
}

fn enter_next_level(
    trigger: Trigger<Pointer<Click>>,
    button_query: Query<&InteractionDisabled, With<Button>>,
    mut selector_query: Query<&mut UpgradeSelector>,
    mut player_deck: ResMut<PlayerDeck>,
    mut upgrade_history: ResMut<UpgradeHistory>,
    mut level: NextMut<Level>,
) {
    let target = r!(trigger.get_target());
    let disabled = r!(button_query.get(target));
    rq!(!disabled.0);

    // Apply upgrades.
    for mut selector in &mut selector_query {
        cq!(selector.selected);

        // Record upgrade history.
        match selector.upgrade {
            Upgrade::NothingPack(_) => upgrade_history.took_nothing_pack = true,
            Upgrade::FireballPack(_) => upgrade_history.took_fireball_pack = true,
            _ => {},
        }

        // Upgrade deck.
        match &mut selector.upgrade {
            Upgrade::FluxCapacitor => player_deck.reactor.extend([Module::EMPTY; 3]),
            Upgrade::QuantumCooler => player_deck.heat_capacity += 8.0,
            Upgrade::AlienAlloy => player_deck.max_health += 50.0,
            Upgrade::NothingPack(modules)
            | Upgrade::RepairPack(modules)
            | Upgrade::MissilePack(modules)
            | Upgrade::LaserPack(modules)
            | Upgrade::FireballPack(modules) => player_deck.storage.append(modules),
        }
    }

    // Enter next level.
    r!(level.get_mut()).0 += 1;
}

fn offered_upgrades(game_assets: &GameAssets, mut upgrades: Vec<Upgrade>) -> impl Bundle {
    (
        Name::new("OfferedUpgrades"),
        Node {
            margin: UiRect::top(Vw(4.0)).with_bottom(Vw(5.2)),
            column_gap: Vw(2.4),
            ..Node::ROW_CENTER
        },
        children![
            upgrade_selector(game_assets, upgrades.remove(0)),
            upgrade_selector(game_assets, upgrades.remove(0)),
            upgrade_selector(game_assets, upgrades.remove(0)),
            upgrade_selector(game_assets, upgrades.remove(0)),
            upgrade_selector(game_assets, upgrades.remove(0)),
            upgrade_selector(game_assets, upgrades.remove(0)),
        ],
    )
}

fn upgrade_selector(game_assets: &GameAssets, upgrade: Upgrade) -> impl Bundle {
    let image = match upgrade {
        Upgrade::FluxCapacitor => &game_assets.upgrade_capacitor,
        Upgrade::QuantumCooler => &game_assets.upgrade_cooler,
        Upgrade::AlienAlloy => &game_assets.upgrade_alloy,
        Upgrade::NothingPack(_) => &game_assets.upgrade_pack_nothing,
        Upgrade::RepairPack(_) => &game_assets.upgrade_pack_repair,
        Upgrade::MissilePack(_) => &game_assets.upgrade_pack_missile,
        Upgrade::LaserPack(_) => &game_assets.upgrade_pack_laser,
        Upgrade::FireballPack(_) => &game_assets.upgrade_pack_fireball,
    }
    .clone();
    // TODO: Write up tooltips.
    let description = match &upgrade {
        Upgrade::FluxCapacitor => "[b]Flux Capacitor[r]\n\n\
            Increase the maximum flux that the reactor can achieve.\n\n\
            [b]Reactor slots:[r] +3"
            .to_string(),
        Upgrade::QuantumCooler => "[b]Quantum Cooler[r]\n\n\
            Allow your reactor modules to activate more before overheating.\n\n\
            [b]Reactor heat capacity:[r] +8"
            .to_string(),
        Upgrade::AlienAlloy => "[b]Alien Alloy[r]\n\n\
            Reinforce your hull with a legendary alloy from another star.\n\n\
            [b]Ship max health:[r] +50"
            .to_string(),
        Upgrade::NothingPack(modules) => {
            format!(
                "[b]Starter Pack[r]\n\n\
                Start your reactor chains with three new Starter modules:\n\n\
                {}\n\
                {}\n\
                {}",
                modules[0].short_description(),
                modules[1].short_description(),
                modules[2].short_description(),
            )
        },
        Upgrade::RepairPack(modules) => {
            format!(
                "[b]Repair Pack[r]\n\n\
                Extend your reactor chains with three new Repair modules:\n\n\
                {}\n\
                {}\n\
                {}",
                modules[0].short_description(),
                modules[1].short_description(),
                modules[2].short_description(),
            )
        },
        Upgrade::MissilePack(modules) => {
            format!(
                "[b]Missile Pack[r]\n\n\
                Extend your reactor chains with three new Missile modules:\n\n\
                {}\n\
                {}\n\
                {}",
                modules[0].short_description(),
                modules[1].short_description(),
                modules[2].short_description(),
            )
        },
        Upgrade::LaserPack(modules) => {
            format!(
                "[b]Laser Pack[r]\n\n\
                Extend your reactor chains with three new Laser modules:\n\n\
                {}\n\
                {}\n\
                {}",
                modules[0].short_description(),
                modules[1].short_description(),
                modules[2].short_description(),
            )
        },
        Upgrade::FireballPack(modules) => {
            format!(
                "[b]Fireball Pack[r]\n\n\
                Finish your reactor chains with three new Fireball modules:\n\n\
                {}\n\
                {}\n\
                {}",
                modules[0].short_description(),
                modules[1].short_description(),
                modules[2].short_description(),
            )
        },
    };

    (
        Name::new("UpgradeSelector"),
        UpgradeSelector::new(upgrade),
        ImageNode::from(image),
        Node {
            width: Vw(6.0417),
            aspect_ratio: Some(1.0),
            ..default()
        },
        NodeOffset::default(),
        InteractionTheme {
            hovered: NodeOffset::new(Val::ZERO, Vw(-0.5)),
            pressed: NodeOffset::new(Val::ZERO, Vw(0.5)),
            ..default()
        },
        BoxShadow::from(ShadowStyle {
            color: Color::BLACK.with_alpha(0.5),
            x_offset: Val::ZERO,
            y_offset: Vw(0.7),
            spread_radius: Vw(0.5),
            blur_radius: Vw(0.5),
        }),
        Backup::<BoxShadow>::default(),
        InteractionSfx,
        InteractionDisabled(false),
        Tooltip::fixed(Anchor::BottomCenter, parse_rich(description)),
        Patch(|entity| {
            entity.observe(toggle_upgrade_selector);
        }),
    )
}

fn toggle_upgrade_selector(
    trigger: Trigger<Pointer<Click>>,
    mut selector_query: Query<(Entity, &mut UpgradeSelector, &mut Node)>,
    mut disabled_query: Query<&mut InteractionDisabled>,
    button_query: Query<Entity, With<IsNextLevelButton>>,
) {
    rq!(matches!(trigger.event.button, PointerButton::Primary));
    let target = r!(trigger.get_target());

    // Toggle the selector.
    let disabled = r!(disabled_query.get(target));
    rq!(!disabled.0);
    let (_, mut selector, mut node) = rq!(selector_query.get_mut(target));
    selector.selected ^= true;
    node.top = if selector.selected {
        Vw(-2.0)
    } else {
        Val::ZERO
    };

    // Update interaction disabling of selectors based on total selected.
    let total_selected = selector_query.iter().filter(|(_, x, _)| x.selected).count();
    for (entity, selector, _) in &mut selector_query {
        let mut disabled = cq!(disabled_query.get_mut(entity));
        if total_selected < 3 {
            disabled.0 = false;
        } else if !selector.selected {
            disabled.0 = true;
        }
    }

    // Update interaction disabling of next level button.
    for entity in &button_query {
        cq!(disabled_query.get_mut(entity)).0 = total_selected < 3;
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct UpgradeSelector {
    upgrade: Upgrade,
    selected: bool,
}

impl Configure for UpgradeSelector {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

impl UpgradeSelector {
    fn new(upgrade: Upgrade) -> Self {
        Self {
            upgrade,
            selected: false,
        }
    }
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
struct UpgradeHistory {
    took_nothing_pack: bool,
    took_fireball_pack: bool,
}

impl Configure for UpgradeHistory {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(StateFlush, Level::ANY.on_exit(reset_upgrade_history));
    }
}

fn reset_upgrade_history(mut upgrade_history: ResMut<UpgradeHistory>) {
    *upgrade_history = default();
}

#[derive(Component, Reflect, Debug, Copy, Clone)]
#[reflect(Component)]
struct IsNextLevelButton;

impl Configure for IsNextLevelButton {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

fn generate_upgrades(
    rng: &mut impl Rng,
    reactor_len: usize,
    upgrade_history: &UpgradeHistory,
) -> Vec<Upgrade> {
    let mut upgrades = vec![];

    // Offer primary upgrades.
    if reactor_len < 18 {
        upgrades.push(Upgrade::FluxCapacitor);
    } else {
        upgrades.push(if rng.r#gen() {
            Upgrade::QuantumCooler
        } else {
            Upgrade::AlienAlloy
        });
    }
    upgrades.push(if rng.r#gen() {
        Upgrade::QuantumCooler
    } else {
        Upgrade::AlienAlloy
    });

    // Offer module packs.
    if !upgrade_history.took_fireball_pack {
        // TODO: Calculate and roll the odds (use level and level config).
        upgrades.push(Upgrade::FireballPack(vec![]));
    }
    if !upgrade_history.took_nothing_pack {
        // TODO: Calculate and roll the odds (use level and level config).
        upgrades.push(Upgrade::NothingPack(vec![]));
    }
    for _ in upgrades.len()..6 {
        // TODO: Get weights from level / level config.
        let choice = cq!(sample_weighted(rng, 5, |x| [1.0, 0.6, 0.7, 0.1, 0.1][x], 1)).index(0);
        upgrades.push(match choice {
            0 => Upgrade::MissilePack(vec![]),
            1 => Upgrade::RepairPack(vec![]),
            2 => Upgrade::LaserPack(vec![]),
            3 => Upgrade::NothingPack(vec![]),
            4 => Upgrade::FireballPack(vec![]),
            _ => unreachable!(),
        });
    }
    upgrades[2..].shuffle(rng);

    // Populate module packs.
    for upgrade in &mut upgrades {
        let (action, modules) = match upgrade {
            Upgrade::NothingPack(modules) => (ModuleAction::Nothing, modules),
            Upgrade::RepairPack(modules) => (ModuleAction::Repair, modules),
            Upgrade::MissilePack(modules) => (ModuleAction::Missile, modules),
            Upgrade::LaserPack(modules) => (ModuleAction::Laser, modules),
            Upgrade::FireballPack(modules) => (ModuleAction::Fireball, modules),
            _ => continue,
        };

        let all_actions = [
            ModuleAction::Missile,
            ModuleAction::Repair,
            ModuleAction::Laser,
            ModuleAction::Nothing,
            ModuleAction::Fireball,
        ];
        let mut other_actions = vec![];
        for _ in 0..3 {
            // TODO: Get weights from level / level config.
            other_actions.push(*c!(all_actions.choose_weighted(rng, |&x| match x {
                x if x == action => 0.0,
                ModuleAction::Missile => 1.0,
                ModuleAction::Repair => 0.6,
                ModuleAction::Laser => 0.7,
                ModuleAction::Nothing => 0.1,
                ModuleAction::Fireball => 0.1,
            })));
        }

        match action {
            ModuleAction::Nothing => {
                for other in other_actions {
                    modules.push(Module::new(action, other));
                }
            },
            ModuleAction::Fireball => {
                for other in other_actions {
                    modules.push(Module::new(other, action));
                }
            },
            _ => {
                modules.push(Module::new(action, other_actions[0]));
                modules.push(Module::new(other_actions[1], action));
                if rng.r#gen() {
                    modules.push(Module::new(action, other_actions[2]));
                } else {
                    modules.push(Module::new(other_actions[2], action));
                }
                modules.shuffle(rng);
            },
        }
    }

    upgrades
}
