use crate::combat::faction::Faction;
use crate::combat::health::OnHeal;
use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::deck::EnemyDeck;
use crate::deck::PlayerDeck;
use crate::level::Level;
use crate::prelude::*;
use crate::projectile::ProjectileConfig;
use crate::screen::gameplay::GameplayAssets;
use crate::ship::Weapon;
use crate::stats::Stats;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<ModuleConfig>, OnAction)>();
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct ModuleConfig {
    pub actions: HashMap<String, ActionInfo>,
}

impl Config for ModuleConfig {
    const FILE: &'static str = "module.ron";

    fn on_load(&mut self, world: &mut World) {
        let asset_server = world.resource::<AssetServer>();
        for action in self.actions.values_mut() {
            action.load(asset_server);
        }
    }

    fn count_progress(&self, asset_server: &AssetServer) -> Progress {
        let mut progress = true.into();
        for action in self.actions.values() {
            progress += action.count_progress(asset_server);
        }
        progress
    }
}

#[derive(Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionInfo {
    pub condition_name: String,
    pub condition_description: String,
    #[serde(rename = "condition_icon")]
    pub condition_icon_path: String,
    #[serde(skip)]
    pub condition_icon: Handle<Image>,
    #[serde(default)]
    pub condition_heat: f32,

    pub effect_name: String,
    pub effect_description: String,
    pub effect_repeat_description: String,
    #[serde(rename = "effect_icon")]
    pub effect_icon_path: String,
    #[serde(skip)]
    pub effect_icon: Handle<Image>,
    #[serde(default)]
    pub effect_projectile: String,
    #[serde(default)]
    pub effect_heal: f32,
    #[serde(default)]
    pub effect_heat: f32,
}

impl ActionInfo {
    fn load(&mut self, asset_server: &AssetServer) {
        self.condition_icon = asset_server.load(&self.condition_icon_path);
        self.effect_icon = asset_server.load(&self.effect_icon_path);
    }

    fn count_progress(&self, asset_server: &AssetServer) -> Progress {
        let mut progress = Progress::default();
        progress += asset_server
            .is_loaded_with_dependencies(&self.condition_icon)
            .into();
        progress += asset_server
            .is_loaded_with_dependencies(&self.effect_icon)
            .into();
        progress
    }
}

#[derive(Reflect, Clone, Default, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Module {
    pub condition: String,
    pub effect: String,
    pub status: ModuleStatus,
    pub heat: f32,
}

impl Module {
    pub const EMPTY: Self = Self {
        condition: String::new(),
        effect: String::new(),
        status: ModuleStatus::SlotEmpty,
        heat: 0.0,
    };

    pub fn new(condition: impl Into<String>, effect: impl Into<String>) -> Self {
        Self {
            condition: condition.into(),
            effect: effect.into(),
            status: ModuleStatus::FaceUp,
            heat: 0.0,
        }
    }

    pub fn short_description(&self, module_config: &ModuleConfig) -> String {
        format!(
            "[b]{}[r] -> [b]{}[r]",
            module_config.actions[&self.condition].condition_name,
            module_config.actions[&self.effect].effect_name,
        )
    }

    pub fn description(
        &self,
        module_config: &ModuleConfig,
        projectile_config: &ProjectileConfig,
        heat_capacity: f32,
    ) -> String {
        let header = self.short_description(module_config);
        let heat = if matches!(self.status, ModuleStatus::SlotOverheated) {
            " (OVERHEATED)".to_string()
        } else {
            format!(" (heat: {}/{})", self.heat, heat_capacity)
        };
        match self.status {
            ModuleStatus::FaceDown => header.to_string(),
            ModuleStatus::SlotEmpty => format!("{header}\n\nEmpty slot"),
            _ => {
                let condition = &module_config.actions[&self.condition];
                let effect = &module_config.actions[&self.effect];
                let body = format!(
                    "{}{}.",
                    condition.condition_description,
                    if self.condition == self.effect {
                        &effect.effect_repeat_description
                    } else {
                        &effect.effect_description
                    },
                );

                let mut stats = String::new();
                if let Some(projectile) =
                    projectile_config.projectiles.get(&effect.effect_projectile)
                {
                    stats += &format!(
                        "\n- [b]Damage:[r] {} times flux",
                        (10.0 * projectile.damage).round() / 10.0,
                    );
                }
                if effect.effect_heal != 0.0 {
                    stats += &format!(
                        "\n- [b]Heal:[r] {} times flux",
                        (10.0 * effect.effect_heal).round() / 10.0,
                    );
                }
                if condition.condition_heat + effect.effect_heat != 0.0 {
                    stats += &format!(
                        "\n- [b]Excess heat:[r] {:+}",
                        (10.0 * (condition.condition_heat + effect.effect_heat)).round() / 10.0,
                    )
                }
                if !stats.is_empty() {
                    stats = format!("\n{stats}");
                }

                format!("{header}{heat}\n\n{body}{stats}")
            },
        }
    }
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
    pub action: String,
    pub source: Entity,
    pub target: Entity,
}

impl Configure for OnAction {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(perform_action);
    }
}

fn perform_action(
    trigger: Trigger<OnAction>,
    mut commands: Commands,
    module_config: ConfigRef<ModuleConfig>,
    projectile_config: ConfigRef<ProjectileConfig>,
    player_deck: Res<PlayerDeck>,
    enemy_deck: Res<EnemyDeck>,
    game_assets: Res<GameplayAssets>,
    audio_settings: Res<AudioSettings>,
    ship_query: Query<(&Children, &Faction)>,
    children_query: Query<&Children>,
    weapon_query: Query<&GlobalTransform, With<Weapon>>,
    mut stats: ResMut<Stats>,
) {
    let projectile_config = r!(projectile_config.get());
    let module_config = r!(module_config.get());
    let action = r!(module_config.actions.get(&trigger.action));

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
    let weapon_gt = r!(weapons.choose(rng));
    let weapon_transform = weapon_gt.compute_transform();

    // Determine flux.
    let is_player = faction == Faction::Player;
    let flux = match is_player {
        true => player_deck.flux,
        false => enemy_deck.flux,
    };

    // Spawn projectile.
    if let Some(projectile) = projectile_config.projectiles.get(&action.effect_projectile) {
        commands.spawn((
            projectile.generate(rng, faction, flux, weapon_transform, trigger.target),
            DespawnOnExitState::<Level>::default(),
        ));
        if let Some(spawn_sfx) = &projectile.spawn_sfx {
            commands.spawn((
                sfx_audio(&audio_settings, spawn_sfx.clone(), 1.0),
                DespawnOnExitState::<Level>::default(),
            ));
        }
    }

    // Heal.
    if action.effect_heal > f32::EPSILON {
        commands
            .entity(trigger.source)
            .trigger(OnHeal(action.effect_heal * flux));
        commands.spawn((
            sfx_audio(
                &audio_settings,
                game_assets.repair_sfx.clone(),
                2f32.powf(1.0 / 12.0 * rng.gen_range(0..12) as f32),
            ),
            DespawnOnExitState::<Level>::default(),
        ));
    }

    *stats.actions.entry_ref(&trigger.action).or_default() += 1;
}
