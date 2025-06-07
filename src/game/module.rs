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
use crate::game::ship::IsWeapon;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<OnModuleAction>();
}

#[derive(Reflect, Copy, Clone, Default, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Module {
    pub condition: ModuleAction,
    pub effect: ModuleAction,
    pub status: ModuleStatus,
    pub heat: f32,
}

impl Module {
    pub const EMPTY: Self = Self {
        condition: ModuleAction::Nothing,
        effect: ModuleAction::Nothing,
        status: ModuleStatus::SlotEmpty,
        heat: 0.0,
    };

    pub fn new(condition: ModuleAction, effect: ModuleAction) -> Self {
        Self {
            condition,
            effect,
            status: ModuleStatus::FaceUp,
            heat: 0.0,
        }
    }

    pub fn description(&self) -> RichText {
        let header = "[b]Reactor module[r]";
        let heat = if matches!(self.status, ModuleStatus::SlotOverheated) {
            " (OVERHEATED)".to_string()
        } else if self.heat > f32::EPSILON {
            format!(" ({} heat)", self.heat)
        } else {
            "".to_string()
        };
        RichText::from_sections(parse_rich(match self.status {
            ModuleStatus::FaceDown => header.to_string(),
            ModuleStatus::SlotEmpty => format!("{header}\n\nEmpty slot"),
            _ => {
                let condition = match self.condition {
                    ModuleAction::Nothing => "Unconditionally ",
                    ModuleAction::Missile => "After firing a missile, ",
                    ModuleAction::Laser => "After firing a laser, ",
                    ModuleAction::Fireball => "After breathing fire, ",
                    ModuleAction::Repair => "After repairing the hull, ",
                };
                let effect = match (&self.condition, &self.effect) {
                    (_, ModuleAction::Nothing) => "do nothing",
                    (ModuleAction::Missile, ModuleAction::Missile) => "fire another missile",
                    (_, ModuleAction::Missile) => "fire a missile",
                    (ModuleAction::Laser, ModuleAction::Laser) => "fire another laser",
                    (_, ModuleAction::Laser) => "fire a laser",
                    (ModuleAction::Fireball, ModuleAction::Fireball) => "breathe more fire",
                    (_, ModuleAction::Fireball) => "breathe fire",
                    (ModuleAction::Repair, ModuleAction::Repair) => "repair the hull again",
                    (_, ModuleAction::Repair) => "repair the hull",
                };
                format!("{header}{heat}\n\n{condition}{effect}.")
            },
        }))
    }
}

#[derive(Reflect, Serialize, Deserialize, Copy, Clone, Default, Eq, PartialEq, Debug)]
pub enum ModuleAction {
    #[default]
    Nothing,
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
pub struct OnModuleAction(pub ModuleAction);

impl Configure for OnModuleAction {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(on_module_action);
    }
}

fn on_module_action(
    trigger: Trigger<OnModuleAction>,
    mut commands: Commands,
    player_deck: Res<PlayerDeck>,
    enemy_deck: Res<EnemyDeck>,
    projectile_config: ConfigRef<ProjectileConfig>,
    game_assets: Res<GameAssets>,
    audio_settings: Res<AudioSettings>,
    ship_query: Query<(&Children, &Faction)>,
    children_query: Query<&Children>,
    weapon_query: Query<&GlobalTransform, With<IsWeapon>>,
) {
    let projectile_config = r!(projectile_config.get());

    // Choose a weapon on the ship.
    let rng = &mut thread_rng();
    let ship = r!(trigger.get_target());
    let (children, &faction) = r!(ship_query.get(ship));
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
    let flux = match faction {
        Faction::Player => player_deck.flux,
        Faction::Enemy => enemy_deck.flux,
    };

    // Perform action.
    match trigger.0 {
        ModuleAction::Missile => {
            commands.spawn((
                missile(
                    rng,
                    projectile_config,
                    &game_assets,
                    faction,
                    flux,
                    weapon_transform,
                ),
                DespawnOnExitState::<Level>::default(),
            ));
        },

        ModuleAction::Laser => {
            commands.spawn((
                laser(
                    rng,
                    projectile_config,
                    &game_assets,
                    faction,
                    flux,
                    weapon_transform,
                ),
                DespawnOnExitState::<Level>::default(),
            ));
            commands.spawn((
                sfx_audio(&audio_settings, game_assets.laser_spawn_sfx.clone(), 1.0),
                DespawnOnExitState::<Level>::default(),
            ));
        },

        ModuleAction::Fireball => {
            commands.spawn((
                fireball(
                    rng,
                    projectile_config,
                    &game_assets,
                    faction,
                    flux,
                    weapon_transform,
                ),
                DespawnOnExitState::<Level>::default(),
            ));
            commands.spawn((
                sfx_audio(&audio_settings, game_assets.fireball_spawn_sfx.clone(), 1.0),
                DespawnOnExitState::<Level>::default(),
            ));
        },

        ModuleAction::Repair => {
            commands.entity(ship).trigger(OnHeal(flux));
            commands.spawn((
                sfx_audio(
                    &audio_settings,
                    game_assets.repair_sfx.clone(),
                    2f32.powf(1.0 / 12.0 * thread_rng().gen_range(0..12) as f32),
                ),
                DespawnOnExitState::<Level>::default(),
            ));
        },

        _ => {},
    }
}
