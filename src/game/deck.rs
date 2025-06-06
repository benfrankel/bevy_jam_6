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
    pub initial_player_deck: PlayerDeck,
}

impl Config for DeckConfig {
    const FILE: &'static str = "deck.ron";
}

#[derive(Resource, Reflect, Serialize, Deserialize, Default, Clone, Debug)]
#[reflect(Resource)]
#[serde(deny_unknown_fields, default)]
pub struct PlayerDeck {
    // Ship:
    pub max_health: f32,

    // Helm:
    pub storage: Vec<Module>,
    pub hand: Vec<Module>,
    pub selected_idx: usize,
    pub just_drawn: Option<bool>,

    // Reactor:
    pub flux: f32,
    pub heat_capacity: f32,
    pub reactor: Vec<Module>,
    pub last_effect: ModuleAction,
    pub last_touched_idx: Option<usize>,
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
        for slot in &mut self.reactor {
            cq!(!matches!(slot.status, ModuleStatus::SlotEmpty));
            slot.status = ModuleStatus::FaceUp;
            slot.heat = 0.0;
            self.storage.push(*slot);
        }
        self.storage.append(&mut self.hand);

        // Create a new deck from storage and number of reactor slots.
        *self = Self {
            max_health: self.max_health,
            heat_capacity: self.heat_capacity,
            storage: core::mem::take(&mut self.storage),
            reactor: vec![Module::EMPTY; self.reactor.len()],
            ..default()
        };

        // Perform setup and select the module in the middle.
        while self.step_setup() {}
        self.selected_idx = 2;
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
        self.just_drawn = Some(true);
    }

    /// Try to play the currently selected module from hand to reactor,
    /// returning false if it's not possible.
    pub fn play_selected(&mut self) -> bool {
        rq!(!self.hand.is_empty() && !self.reactor.is_empty());
        let slot_idx = rq!(self.next_available_slot());

        // Remove the selected module from hand.
        let idx = self.selected_idx;
        let mut selected = self.hand.remove(idx);
        self.selected_idx = self
            .selected_idx
            .clamp(0, self.hand.len().saturating_sub(1));

        // Place it in the next available reactor slot.
        let slot = &mut self.reactor[slot_idx];
        if !matches!(slot.status, ModuleStatus::SlotEmpty) {
            slot.status = ModuleStatus::FaceUp;
            slot.heat = 0.0;
            self.storage.push(*slot);
        }
        selected.status = ModuleStatus::SlotInactive;
        *slot = selected;
        self.last_touched_idx = Some(slot_idx);

        true
    }

    /// Find the next available reactor slot to place a module in.
    fn next_available_slot(&self) -> Option<usize> {
        self.reactor
            .iter()
            .position(|slot| matches!(slot.status, ModuleStatus::SlotOverheated))
            .or_else(|| {
                self.reactor
                    .iter()
                    .position(|slot| matches!(slot.status, ModuleStatus::SlotEmpty))
            })
    }

    /// Find the next matching reactor module to trigger.
    fn next_matching_module(&self) -> Option<usize> {
        self.reactor.iter().position(|slot| {
            matches!(slot.status, ModuleStatus::SlotInactive)
                && (matches!(slot.condition, ModuleAction::Nothing)
                    || slot.condition == self.last_effect)
        })
    }

    /// Determine whether the reactor is done yielding actions.
    pub fn is_reactor_done(&self) -> bool {
        self.next_matching_module().is_none()
    }

    /// Simulate one reactor step and get the next action.
    pub fn step_reactor(&mut self) -> Option<ModuleAction> {
        // Activate the first matching module.
        if let Some(idx) = self.next_matching_module() {
            let slot = &mut self.reactor[idx];

            slot.status = ModuleStatus::SlotActive;
            self.last_effect = slot.effect;
            self.flux += 1.0;
            slot.heat += self.flux;
            self.last_touched_idx = Some(idx);

            return Some(slot.effect);
        }

        // If there was no match, reset the reactor.
        self.last_touched_idx = None;
        self.last_effect = ModuleAction::Nothing;
        self.flux = 0.0;
        for slot in &mut self.reactor {
            cq!(matches!(slot.status, ModuleStatus::SlotActive));
            slot.status = ModuleStatus::SlotInactive;
        }

        // Mark any naturally overheated slots.
        for slot in &mut self.reactor {
            cq!(matches!(slot.status, ModuleStatus::SlotInactive) && slot.heat > self.heat_capacity);
            slot.status = ModuleStatus::SlotOverheated;
        }

        // If there are no available slots remaining, artificially mark the hottest slot as overheated.
        if self.next_available_slot().is_none() {
            let slot = r!(self
                .reactor
                .iter_mut()
                .rev()
                .max_by_key(|slot| (slot.heat * 100.0) as i64));
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
    /// The maximum number of actions to be performed on the first round.
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
            flux: 0.0,
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
            self.flux = 1.0;
            None
        } else {
            self.action_idx += 1;
            self.flux += 1.0;
            Some(self.actions[self.action_idx - 1])
        }
    }
}
