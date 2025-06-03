use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Deck>();
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct Deck {
    pub flux: f32,
    pub storage: Vec<Module>,
    pub hand: Vec<Module>,
    pub selected_idx: usize,
    pub reactor: Vec<Module>,
    pub next_slot: usize,
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
            storage: vec![
                Module::new(ModuleAction::Laser, ModuleAction::Heal),
                Module::new(ModuleAction::Missile, ModuleAction::Missile),
                Module::new(ModuleAction::Missile, ModuleAction::Missile),
                Module::new(ModuleAction::Missile, ModuleAction::Missile),
                Module::new(ModuleAction::Missile, ModuleAction::Missile),
                Module::new(ModuleAction::Missile, ModuleAction::Missile),
                Module::new(ModuleAction::Missile, ModuleAction::Missile),
                Module::new(ModuleAction::Missile, ModuleAction::Missile),
            ],
            hand: vec![
                Module::new(ModuleAction::Nothing, ModuleAction::Missile),
                Module::new(ModuleAction::Laser, ModuleAction::Heal),
                Module::new(ModuleAction::Missile, ModuleAction::Missile),
                Module::new(ModuleAction::Heal, ModuleAction::Laser),
                Module::new(ModuleAction::Fire, ModuleAction::Missile),
            ],
            selected_idx: 0,
            reactor: vec![Module::EMPTY; 9],
            next_slot: 0,
        }
    }
}

impl Deck {
    /// Discard all cards in the deck.
    pub fn reset(&mut self) {
        self.flux = 0.0;
        for slot in &mut self.reactor {
            *slot = Module::EMPTY;
        }
        self.storage.append(&mut self.hand);
        self.selected_idx = 0;
    }

    /// Advance the selected module index by the given step.
    pub fn advance(&mut self, step: isize) {
        self.selected_idx = self
            .selected_idx
            .saturating_add_signed(step)
            .min(self.hand.len().saturating_sub(1));
    }

    /// Play the currently selected module.
    pub fn play(&mut self) {
        rq!(!self.hand.is_empty());

        // Remove the selected module from hand.
        let idx = self.selected_idx;
        let mut selected = self.hand.remove(idx);

        // Place it in the next reactor slot.
        let idx = self.next_slot;
        if !matches!(self.reactor[idx].status, ModuleStatus::SlotEmpty) {
            let mut replaced = self.reactor[idx];
            replaced.status = ModuleStatus::FaceUp;
            self.storage.push(replaced);
        }
        selected.status = ModuleStatus::SlotInactive;
        self.reactor[idx] = selected;
        self.next_slot += 1;
        if self.next_slot >= self.reactor.len() {
            self.next_slot = 0;
        }

        // Draw a new module to hand.
        if !self.storage.is_empty() {
            let idx = thread_rng().gen_range(0..self.storage.len());
            let draw = self.storage.swap_remove(idx);
            self.hand.push(draw);
        }

        // Clamp selected index within new hand size.
        self.selected_idx = self
            .selected_idx
            .clamp(0, self.hand.len().saturating_sub(1));
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
