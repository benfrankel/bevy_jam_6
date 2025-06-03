use crate::game::module::Module;
use crate::game::module::ModuleAction;
use crate::game::module::ModuleStatus;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<DeckConfig>, PlayerDeck, EnemyDeck)>();
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct DeckConfig {
    hand: Vec<Module>,
}

impl Config for DeckConfig {
    const FILE: &'static str = "starter.ron";
}

impl DeckConfig {
    pub fn starter(&self) -> Vec<Module> {
        self.hand.clone()
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct PlayerDeck {
    pub flux: f32,
    pub storage: Vec<Module>,
    pub hand: Vec<Module>,
    pub selected_idx: usize,
    pub reactor: Vec<Module>,
    pub next_slot: usize,
    pub last_effect: ModuleAction,
}

impl Configure for PlayerDeck {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl Default for PlayerDeck {
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
            last_effect: ModuleAction::Nothing,
        }
    }
}

impl PlayerDeck {
    /// Discard all cards in the deck.
    pub fn reset(&mut self) {
        *self = default();
        /*
        self.flux = 0.0;
        for slot in &mut self.reactor {
            *slot = Module::EMPTY;
        }
        self.storage.append(&mut self.hand);
        self.selected_idx = 0;
        */
    }

    /// Advance the selected module index by the given step.
    pub fn advance_selected(&mut self, step: isize) {
        self.selected_idx = self
            .selected_idx
            .saturating_add_signed(step)
            .min(self.hand.len().saturating_sub(1));
    }

    /// Play the currently selected module.
    pub fn play_selected(&mut self) {
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

    /// Whether the deck is done yielding actions.
    pub fn is_done(&self) -> bool {
        !self.reactor.iter().any(|module| {
            matches!(module.status, ModuleStatus::SlotInactive)
                && (matches!(module.condition, ModuleAction::Nothing)
                    || module.condition == self.last_effect)
        })
    }

    /// Simulate one step and get the next action.
    pub fn step(&mut self) -> Option<ModuleAction> {
        // Search for a matching module.
        for module in &mut self.reactor {
            cq!(matches!(module.status, ModuleStatus::SlotInactive));
            cq!(matches!(module.condition, ModuleAction::Nothing)
                || module.condition == self.last_effect);

            // Activate the module.
            module.status = ModuleStatus::SlotActive;
            self.last_effect = module.effect;
            self.flux += 1.0;
            return Some(module.effect);
        }

        // If there was no match, reset the reactor.
        self.last_effect = ModuleAction::Nothing;
        self.flux = 0.0;
        for module in &mut self.reactor {
            cq!(matches!(module.status, ModuleStatus::SlotActive));
            module.status = ModuleStatus::SlotInactive;
        }

        None
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct EnemyDeck {
    pub flux: f32,
    pub actions: Vec<ModuleAction>,
    pub action_idx: usize,
}

impl Configure for EnemyDeck {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl Default for EnemyDeck {
    fn default() -> Self {
        Self {
            flux: 1.0,
            actions: vec![ModuleAction::Missile; 6],
            action_idx: 0,
        }
    }
}

impl EnemyDeck {
    pub fn reset(&mut self) {
        *self = default();
    }

    /// Whether the deck is done yielding actions.
    pub fn is_done(&self) -> bool {
        self.action_idx >= self.actions.len()
    }

    /// Simulate one step and get the next action.
    pub fn step(&mut self) -> Option<ModuleAction> {
        if let Some(&action) = self.actions.get(self.action_idx) {
            self.action_idx += 1;
            Some(action)
        } else {
            self.action_idx = 0;
            None
        }
    }
}
