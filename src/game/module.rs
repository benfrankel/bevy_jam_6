use crate::game::combat::faction::Faction;
use crate::game::combat::health::OnHeal;
use crate::game::deck::EnemyDeck;
use crate::game::deck::PlayerDeck;
use crate::game::level::Level;
use crate::game::missile::MissileAssets;
use crate::game::missile::MissileConfig;
use crate::game::missile::missile;
use crate::game::ship::IsWeapon;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<OnModuleAction>();
}

#[derive(Reflect, Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub struct Module {
    pub condition: ModuleAction,
    pub effect: ModuleAction,
    pub status: ModuleStatus,
}

impl Module {
    pub const EMPTY: Self = Self {
        condition: ModuleAction::Nothing,
        effect: ModuleAction::Nothing,
        status: ModuleStatus::SlotEmpty,
    };

    pub fn new(condition: ModuleAction, effect: ModuleAction) -> Self {
        Self {
            condition,
            effect,
            status: ModuleStatus::FaceUp,
        }
    }
}

#[derive(Reflect, Serialize, Deserialize, Copy, Clone, Default, Eq, PartialEq, Debug)]
pub enum ModuleAction {
    #[default]
    Nothing,
    Missile,
    Laser,
    Fire,
    Heal,
}

#[derive(Reflect, Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub enum ModuleStatus {
    #[default]
    FaceUp,
    FaceDown,
    SlotEmpty,
    SlotInactive,
    SlotActive,
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
    missile_config: ConfigRef<MissileConfig>,
    missile_assets: Res<MissileAssets>,
    ship_query: Query<(&Children, &Faction)>,
    weapon_query: Query<&GlobalTransform, With<IsWeapon>>,
) {
    // Choose a weapon on the ship.
    let rng = &mut thread_rng();
    let ship = r!(trigger.get_target());
    let (children, &faction) = r!(ship_query.get(ship));
    let weapons = children
        .iter()
        .filter_map(|entity| weapon_query.get(entity).ok())
        .collect::<Vec<_>>();
    let weapon_gt = r!(weapons.choose(&mut thread_rng()));
    let weapon_transform = weapon_gt.compute_transform();

    // Determine flux.
    let flux = match faction {
        Faction::Player => player_deck.flux,
        Faction::Enemy => enemy_deck.flux,
    };

    match trigger.0 {
        ModuleAction::Missile => {
            let missile_config = r!(missile_config.get());
            commands.spawn((
                missile(
                    rng,
                    missile_config,
                    &missile_assets,
                    faction,
                    flux,
                    weapon_transform,
                ),
                DespawnOnExitState::<Level>::default(),
            ));
        },

        // TODO: Implement this.
        ModuleAction::Laser => {},

        // TODO: Implement this.
        ModuleAction::Fire => {},

        ModuleAction::Heal => {
            commands.entity(ship).trigger(OnHeal(1. * flux));
        },

        _ => {},
    }
}
