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
    pub reactor: Vec<(Module, f32)>,
    pub last_effect: ModuleAction,
}

impl Configure for PlayerDeck {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl PlayerDeck {
    /// Reset deck.
    pub fn reset(&mut self) {
        // Discard modules from reactor / hand to storage.
        for (slot, _) in &mut self.reactor {
            cq!(!matches!(slot.status, ModuleStatus::SlotEmpty));
            slot.status = ModuleStatus::FaceUp;
            self.storage.push(*slot);
            slot.status = ModuleStatus::SlotEmpty;
        }
        self.storage.append(&mut self.hand);

        // Create a new deck from storage and number of reactor slots.
        *self = Self {
            storage: core::mem::take(&mut self.storage),
            reactor: vec![(Module::EMPTY, 0.0); self.reactor.len()],
            ..default()
        };
    }

    /// Shuffle storage.
    pub fn shuffle(&mut self, rng: &mut impl Rng) {
        self.storage.shuffle(rng);
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
        let slot_idx = rq!(self.next_slot_idx());

        // Remove the selected module from hand.
        let idx = self.selected_idx;
        let mut selected = self.hand.remove(idx);
        self.selected_idx = self
            .selected_idx
            .clamp(0, self.hand.len().saturating_sub(1));

        // Place it in the next available reactor slot.
        let (slot, heat) = &mut self.reactor[slot_idx];
        if !matches!(slot.status, ModuleStatus::SlotEmpty) {
            slot.status = ModuleStatus::FaceUp;
            self.storage.push(*slot);
        }
        selected.status = ModuleStatus::SlotInactive;
        *slot = selected;
        *heat = 0.0;

        true
    }

    /// Determine the next available reactor slot.
    fn next_slot_idx(&self) -> Option<usize> {
        self.reactor
            .iter()
            .position(|(slot, _)| matches!(slot.status, ModuleStatus::SlotEmpty))
            .or_else(|| {
                self.reactor
                    .iter()
                    .position(|(slot, _)| matches!(slot.status, ModuleStatus::SlotOverheated))
            })
    }

    /// Determine whether the reactor is done yielding actions.
    pub fn is_reactor_done(&self) -> bool {
        !self.reactor.iter().any(|(slot, _)| {
            matches!(slot.status, ModuleStatus::SlotInactive)
                && (matches!(slot.condition, ModuleAction::Nothing)
                    || slot.condition == self.last_effect)
        })
    }

    /// Simulate one reactor step and get the next action.
    pub fn step_reactor(&mut self) -> Option<ModuleAction> {
        // Search for a matching module.
        for (slot, heat) in &mut self.reactor {
            cq!(matches!(slot.status, ModuleStatus::SlotInactive));
            cq!(matches!(slot.condition, ModuleAction::Nothing)
                || slot.condition == self.last_effect);

            // Activate the module.
            slot.status = ModuleStatus::SlotActive;
            self.last_effect = slot.effect;
            self.flux += 1.0;
            *heat += self.flux;
            return Some(slot.effect);
        }

        // If there was no match, reset the reactor.
        self.last_effect = ModuleAction::Nothing;
        self.flux = 0.0;
        for (slot, _) in &mut self.reactor {
            cq!(matches!(slot.status, ModuleStatus::SlotActive));
            slot.status = ModuleStatus::SlotInactive;
        }

        // And if the reactor is full, mark at least one module as overheated.
        if self
            .reactor
            .iter()
            .all(|(slot, _)| matches!(slot.status, ModuleStatus::SlotInactive))
        {
            let (slot, _) = r!(self
                .reactor
                .iter_mut()
                .rev()
                .max_by_key(|(_, heat)| (heat * 100.0) as i64));
            slot.status = ModuleStatus::SlotOverheated;
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
    pub action_limit: usize,
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
            action_limit: 1,
        }
    }
}

impl EnemyDeck {
    pub fn reset(&mut self) {
        *self = default();
    }

    /// Determine whether the deck is done yielding actions.
    pub fn is_done(&self, round: usize) -> bool {
        self.action_idx
            >= self
                .actions
                .len()
                .min(self.action_limit.saturating_add(round))
    }

    /// Simulate one step and get the next action.
    pub fn step(&mut self, round: usize) -> Option<ModuleAction> {
        if self.is_done(round) {
            self.action_idx = 0;
            None
        } else {
            self.action_idx += 1;
            Some(self.actions[self.action_idx - 1])
        }
    }
}
