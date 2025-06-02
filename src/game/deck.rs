use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Deck>();
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct Deck {
    pub flux: f32,
    pub reactor: Vec<Module>,
    pub draw_pile: Vec<Module>,
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
            draw_pile: vec![],
            hand: vec![],
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
        self.draw_pile.extend(self.hand.drain(..));
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
