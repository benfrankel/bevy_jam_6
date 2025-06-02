use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Deck>();
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct Deck {
    pub flux: f32,
    pub reactor: Vec<Module>,
    pub storage: Vec<Module>,
    pub hand: Vec<Module>,
    pub focused_idx: usize,
}

impl Configure for Deck {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self {
            flux: 0.0,
            reactor: vec![Module::EMPTY; 9],
            storage: vec![
                Module::new(ModuleAction::Laser, ModuleAction::Heal),
                Module::new(ModuleAction::Missile, ModuleAction::Missile),
            ],
            hand: vec![
                Module::new(ModuleAction::Nothing, ModuleAction::Missile),
                Module::new(ModuleAction::Laser, ModuleAction::Heal),
                Module::new(ModuleAction::Missile, ModuleAction::Missile),
                Module::new(ModuleAction::Heal, ModuleAction::Laser),
                Module::new(ModuleAction::Fire, ModuleAction::Missile),
            ],
            focused_idx: 0,
        }
    }
}

impl Deck {
    pub fn reset(&mut self) {
        self.flux = 0.0;
        for slot in &mut self.reactor {
            *slot = Module::EMPTY;
        }
        self.storage.append(&mut self.hand);
        self.focused_idx = 0;
    }
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
