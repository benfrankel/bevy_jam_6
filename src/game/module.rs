use crate::game::combat::faction::Faction;
use crate::game::level::Level;
use crate::game::missile::MissileAssets;
use crate::game::missile::missile;
use crate::game::ship::IsWeapon;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<OnModuleAction>();
}

#[derive(Reflect, Copy, Clone, Default, Debug)]
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

#[derive(Reflect, Copy, Clone, Default, Debug)]
pub enum ModuleAction {
    #[default]
    Nothing,
    Missile,
    Laser,
    Fire,
    Heal,
}

#[derive(Reflect, Copy, Clone, Default, Debug)]
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
    missile_assets: Res<MissileAssets>,
    ship_query: Query<(&Children, &Faction)>,
    weapon_query: Query<&GlobalTransform, With<IsWeapon>>,
) {
    match trigger.0 {
        ModuleAction::Missile => {
            // Choose a weapon on the ship.
            let ship = r!(trigger.get_target());
            let (children, &faction) = r!(ship_query.get(ship));
            let weapons = children
                .iter()
                .filter_map(|entity| weapon_query.get(entity).ok())
                .collect::<Vec<_>>();
            let gt = **r!(weapons.choose(&mut thread_rng()));

            // Spawn a missile from the chosen weapon.
            commands.spawn((
                missile(&missile_assets, faction, thread_rng().gen_range(0.0..15.0)),
                gt.compute_transform(),
                gt,
                DespawnOnExitState::<Level>::default(),
            ));
        },

        // TODO: Implement this.
        ModuleAction::Laser => {},

        // TODO: Implement this.
        ModuleAction::Fire => {},

        // TODO: Implement this.
        ModuleAction::Heal => {},

        _ => {},
    }
}
