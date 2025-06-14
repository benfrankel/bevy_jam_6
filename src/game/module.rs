use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::game::GameAssets;
use crate::game::combat::faction::Faction;
use crate::game::combat::health::OnHeal;
use crate::game::deck::EnemyDeck;
use crate::game::deck::PlayerDeck;
use crate::game::level::Level;
use crate::game::projectile::ProjectileConfig;
use crate::game::projectile::fireball::fireball;
use crate::game::projectile::laser::laser;
use crate::game::projectile::missile::missile;
use crate::game::ship::Weapon;
use crate::game::stats::Stats;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<OnAction>();
}

#[derive(Reflect, Copy, Clone, Default, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Module {
    pub condition: Action,
    pub effect: Action,
    pub status: ModuleStatus,
    pub heat: f32,
}

impl Module {
    pub const EMPTY: Self = Self {
        condition: Action::Start,
        effect: Action::Start,
        status: ModuleStatus::SlotEmpty,
        heat: 0.0,
    };

    pub fn new(condition: Action, effect: Action) -> Self {
        Self {
            condition,
            effect,
            status: ModuleStatus::FaceUp,
            heat: 0.0,
        }
    }

    pub fn short_description(&self) -> String {
        let condition = match self.condition {
            Action::Start => "[b]Start[r] -> ",
            Action::Missile => "[b]Missile[r] -> ",
            Action::Laser => "[b]Laser[r] -> ",
            Action::Fireball => "[b]Fireball[r] -> ",
            Action::Repair => "[b]Repair[r] -> ",
        };
        let effect = match self.effect {
            Action::Start => "[b]Nothing[r]",
            Action::Missile => "[b]Missile[r]",
            Action::Laser => "[b]Laser[r]",
            Action::Fireball => "[b]Fireball[r]",
            Action::Repair => "[b]Repair[r]",
        };
        format!("{condition}{effect}")
    }

    pub fn description(&self, heat_capacity: f32) -> String {
        let header = "[b]Reactor module[r]";
        let heat = if matches!(self.status, ModuleStatus::SlotOverheated) {
            " (OVERHEATED)".to_string()
        } else {
            format!(" (heat: {}/{})", self.heat, heat_capacity)
        };
        match self.status {
            ModuleStatus::FaceDown => header.to_string(),
            ModuleStatus::SlotEmpty => format!("{header}\n\nEmpty slot"),
            _ => {
                let condition = match self.condition {
                    Action::Start => "at the start of a new chain, ",
                    Action::Missile => "after launching a missile, ",
                    Action::Laser => "after firing a laser, ",
                    Action::Fireball => "after unleashing a fireball, ",
                    Action::Repair => "after repairing the hull, ",
                };
                let effect = match (&self.condition, &self.effect) {
                    (_, Action::Start) => "end the chain",
                    (Action::Missile, Action::Missile) => "launch another missile",
                    (_, Action::Missile) => "launch a missile",
                    (Action::Laser, Action::Laser) => "fire another laser",
                    (_, Action::Laser) => "fire a laser",
                    (Action::Fireball, Action::Fireball) => {
                        "unleash another fireball and end the chain"
                    },
                    (_, Action::Fireball) => "unleash a fireball and end the chain",
                    (Action::Repair, Action::Repair) => "repair the hull again",
                    (_, Action::Repair) => "repair the hull",
                };
                let body = format!("{condition}{effect}.");
                let mut chars = body.chars();
                let body = match chars.next() {
                    Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                    None => String::new(),
                };

                let stats = match self.effect {
                    Action::Start => "",
                    Action::Missile => "\n\n[b]Damage:[r] 1 times flux",
                    Action::Laser => "\n\n[b]Damage:[r] 2 times flux",
                    Action::Fireball => "\n\n[b]Damage:[r] 8 times flux",
                    Action::Repair => "\n\n[b]Heal:[r] 2 times flux",
                };

                format!("{header}{heat} - Right click to remove\n\n> {body}{stats}")
            },
        }
    }
}

#[derive(Reflect, Serialize, Deserialize, Copy, Clone, Default, Eq, PartialEq, Debug)]
pub enum Action {
    #[default]
    Start,
    Missile,
    Laser,
    Fireball,
    Repair,
}

#[derive(Reflect, Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub enum ModuleStatus {
    #[default]
    FaceUp,
    FaceDown,
    SlotEmpty,
    SlotInactive,
    SlotActive,
    SlotOverheated,
}

#[derive(Event, Reflect, Debug)]
pub struct OnAction {
    pub action: Action,
    pub source: Entity,
    pub target: Entity,
}

impl Configure for OnAction {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(on_module_action);
    }
}

fn on_module_action(
    trigger: Trigger<OnAction>,
    mut commands: Commands,
    player_deck: Res<PlayerDeck>,
    enemy_deck: Res<EnemyDeck>,
    projectile_config: ConfigRef<ProjectileConfig>,
    game_assets: Res<GameAssets>,
    audio_settings: Res<AudioSettings>,
    ship_query: Query<(&Children, &Faction)>,
    children_query: Query<&Children>,
    weapon_query: Query<&GlobalTransform, With<Weapon>>,
    mut stats: ResMut<Stats>,
) {
    let projectile_config = r!(projectile_config.get());

    // Choose a weapon on the ship.
    let rng = &mut thread_rng();
    let (children, &faction) = r!(ship_query.get(trigger.source));
    let mut weapons = Vec::<&_>::new();
    for &child in children {
        weapons.extend(weapon_query.get(child));
        for &child in children_query.get(child).into_iter().flatten() {
            weapons.extend(weapon_query.get(child));
        }
    }
    let weapon_gt = r!(weapons.choose(&mut thread_rng()));
    let weapon_transform = weapon_gt.compute_transform();

    // Determine flux.
    let is_player = faction == Faction::Player;
    let flux = match is_player {
        true => player_deck.flux,
        false => enemy_deck.flux,
    };

    // Perform action.
    match trigger.action {
        Action::Missile => {
            commands.spawn((
                missile(
                    rng,
                    projectile_config,
                    &game_assets,
                    faction,
                    flux,
                    weapon_transform,
                    trigger.target,
                ),
                DespawnOnExitState::<Level>::default(),
            ));
            commands.spawn((
                sfx_audio(&audio_settings, game_assets.missile_spawn_sfx.clone(), 1.0),
                DespawnOnExitState::<Level>::default(),
            ));

            if is_player {
                stats.missiles += 1;
            }
        },

        Action::Laser => {
            commands.spawn((
                laser(
                    rng,
                    projectile_config,
                    &game_assets,
                    faction,
                    flux,
                    weapon_transform,
                    trigger.target,
                ),
                DespawnOnExitState::<Level>::default(),
            ));
            commands.spawn((
                sfx_audio(&audio_settings, game_assets.laser_spawn_sfx.clone(), 1.0),
                DespawnOnExitState::<Level>::default(),
            ));

            if is_player {
                stats.lasers += 1;
            }
        },

        Action::Fireball => {
            commands.spawn((
                fireball(
                    rng,
                    projectile_config,
                    &game_assets,
                    faction,
                    flux,
                    weapon_transform,
                    trigger.target,
                ),
                DespawnOnExitState::<Level>::default(),
            ));
            commands.spawn((
                sfx_audio(&audio_settings, game_assets.fireball_spawn_sfx.clone(), 1.0),
                DespawnOnExitState::<Level>::default(),
            ));

            if is_player {
                stats.fireballs += 1;
            }
        },

        Action::Repair => {
            commands.entity(trigger.source).trigger(OnHeal(2.0 * flux));
            commands.spawn((
                sfx_audio(
                    &audio_settings,
                    game_assets.repair_sfx.clone(),
                    2f32.powf(1.0 / 12.0 * thread_rng().gen_range(0..12) as f32),
                ),
                DespawnOnExitState::<Level>::default(),
            ));

            if is_player {
                stats.repairs += 1;
            }
        },

        _ => {},
    }
}
