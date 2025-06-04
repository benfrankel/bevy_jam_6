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
    initial_storage: Vec<(ModuleAction, ModuleAction)>,
}

impl Config for DeckConfig {
    const FILE: &'static str = "deck.ron";
}

impl DeckConfig {
    pub fn initial_player_deck(&self) -> PlayerDeck {
        PlayerDeck {
            storage: self
                .initial_storage
                .iter()
                .copied()
                .map(|(x, y)| Module::new(x, y))
                .collect(),
            reactor: vec![Module::EMPTY; 3],
            ..default()
        }
    }
}

#[derive(Resource, Reflect, Debug, Default)]
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

impl PlayerDeck {
    /// Discard all cards in the deck.
    pub fn reset(&mut self) {
        *self = default();
    }

    /// Advance the selected module index by the given step.
    pub fn advance_selected(&mut self, step: isize) {
        self.selected_idx = self
            .selected_idx
            .saturating_add_signed(step)
            .min(self.hand.len().saturating_sub(1));
    }

    /// Draw a random module from storage to hand.
    pub fn draw(&mut self) {
        rq!(!self.storage.is_empty());
        let idx = thread_rng().gen_range(0..self.storage.len());
        let draw = self.storage.swap_remove(idx);
        self.hand.push(draw);
    }

    /// Try to play the currently selected module from hand to reactor,
    /// returning false if it's not possible.
    pub fn play_selected(&mut self) -> bool {
        rq!(!self.hand.is_empty() && !self.reactor.is_empty());

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

        // Clamp selected index within new hand size.
        self.selected_idx = self
            .selected_idx
            .clamp(0, self.hand.len().saturating_sub(1));

        true
    }

    /// Determine whether the reactor is done yielding actions.
    pub fn is_reactor_done(&self) -> bool {
        !self.reactor.iter().any(|module| {
            matches!(module.status, ModuleStatus::SlotInactive)
                && (matches!(module.condition, ModuleAction::Nothing)
                    || module.condition == self.last_effect)
        })
    }

    /// Simulate one reactor step and get the next action.
    pub fn step_reactor(&mut self) -> Option<ModuleAction> {
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

    /// Determines whether setup is complete.
    pub fn is_setup_done(&self) -> bool {
        self.storage.is_empty() || self.hand.len() >= 5
    }

    /// Steps setting up the deck, returning false if setup was already complete.
    pub fn step_setup(&mut self) -> bool {
        if !self.storage.is_empty() && self.hand.len() < 5 {
            self.draw();
        } else {
            return false;
        }

        true
    }
}

#[derive(Resource, Reflect, Serialize, Deserialize, Clone, Debug)]
#[reflect(Resource)]
#[serde(deny_unknown_fields, default)]
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
            actions: vec![],
            action_idx: 0,
        }
    }
}

impl EnemyDeck {
    pub fn reset(&mut self) {
        *self = default();
    }

    /// Determine whether the deck is done yielding actions.
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
