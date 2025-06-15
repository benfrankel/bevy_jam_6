use std::collections::VecDeque;

use crate::game::module::Module;
use crate::game::module::ModuleStatus;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<DeckConfig>, PlayerDeck, EnemyDeck)>();
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct DeckConfig {
    pub player_decks: Vec<PlayerDeck>,
}

impl Config for DeckConfig {
    const FILE: &'static str = "deck.ron";
}

#[derive(Resource, Reflect, Serialize, Deserialize, Default, Clone, Debug)]
#[reflect(Resource)]
#[serde(deny_unknown_fields, default)]
pub struct PlayerDeck {
    // Ship:
    pub name: String,
    pub max_health: f32,
    pub heat_capacity: f32,
    pub hand_size: usize,
    pub weapons: Vec<Module>,

    // Modules:
    pub storage: Vec<Module>,
    pub hand: Vec<Module>,
    pub reactor: Vec<Module>,

    // Helm:
    pub hand_idx: usize,
    pub just_used_storage: bool,

    // Reactor:
    pub flux: f32,
    pub chain: f32,
    pub action_queue: VecDeque<usize>,
    pub last_action: String,
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
    pub fn reset(&mut self, rng: &mut impl Rng) {
        // Discard modules from reactor / hand to storage.
        for idx in 0..self.reactor.len() {
            self.discard_module(idx);
        }
        self.storage.append(&mut self.hand);

        // Reset turn-based state.
        self.just_used_storage = false;
        self.flux = 0.0;
        self.chain = 0.0;
        self.action_queue.clear();
        self.last_action.clear();
        self.last_touched_idx = None;

        // Perform setup phase and select the middle module.
        let weapons = self.weapons.clone();
        for weapon in weapons {
            let draw_idx = cq!(self
                .storage
                .iter()
                .position(|x| x.condition == weapon.condition && x.effect == weapon.effect));
            self.draw_from_idx(draw_idx);
        }
        while self.step_setup(rng) {}
        self.hand_idx = self.hand.len() / 2;
    }

    /// Advance the selected module index by the given step.
    pub fn advance_selected(&mut self, step: isize) {
        self.hand_idx = self
            .hand_idx
            .saturating_add_signed(step)
            .min(self.hand.len().saturating_sub(1));
    }

    /// Draw the next module from storage to hand.
    pub fn draw_random(&mut self, rng: &mut impl Rng) {
        rq!(!self.storage.is_empty());
        let draw_idx = rng.gen_range(0..self.storage.len());
        self.draw_from_idx(draw_idx);
    }

    /// Draw the specified module from storage to hand.
    fn draw_from_idx(&mut self, idx: usize) {
        let draw = self.storage.swap_remove(idx);
        self.hand.push(draw);
        self.just_used_storage = true;
    }

    /// Discard a module from the reactor to storage.
    pub fn discard_module(&mut self, idx: usize) {
        rq!(!matches!(self.reactor[idx].status, ModuleStatus::SlotEmpty));
        self.reactor[idx].status = ModuleStatus::SlotEmpty;

        let mut slot = self.reactor[idx].clone();
        slot.status = ModuleStatus::FaceUp;
        slot.heat = 0.0;
        self.storage.push(slot);
        self.just_used_storage = true;
    }

    /// Try to play the currently selected module from hand to reactor,
    /// returning false if it's not possible.
    pub fn play_selected(&mut self) -> bool {
        rq!(!self.hand.is_empty() && !self.reactor.is_empty());
        let slot_idx = rq!(self.next_available_slot());

        // Remove the selected module from hand.
        let idx = self.hand_idx;
        let mut selected = self.hand.remove(idx);
        self.hand_idx = self.hand_idx.clamp(0, self.hand.len().saturating_sub(1));

        // Place it in the next available reactor slot.
        self.discard_module(slot_idx);
        selected.status = ModuleStatus::SlotInactive;
        self.reactor[slot_idx] = selected;
        self.last_touched_idx = Some(slot_idx);

        true
    }

    /// Try to discard the currently selected module from hand to storage,
    /// returning false if it's not possible.
    pub fn discard_selected(&mut self, rng: &mut impl Rng) -> bool {
        rq!(!self.hand.is_empty());

        // Remove selected module from hand.
        let idx = self.hand_idx;
        let selected = self.hand.remove(idx);
        self.hand_idx = self.hand_idx.clamp(0, self.hand.len().saturating_sub(1));

        // Insert it into storage.
        let storage_idx = rng.gen_range(0..=self.storage.len() / 2);
        self.storage.insert(storage_idx, selected);
        self.just_used_storage = true;

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
        self.reactor
            .iter()
            .position(|slot| {
                matches!(slot.status, ModuleStatus::SlotInactive)
                    && slot.condition == self.last_action
            })
            .or_else(|| {
                self.reactor.iter().position(|slot| {
                    matches!(slot.status, ModuleStatus::SlotInactive) && slot.condition.is_empty()
                })
            })
    }

    /// Determine whether the reactor is done powering up.
    pub fn is_reactor_done(&self) -> bool {
        self.next_matching_module().is_none()
    }

    /// Simulate one step powering up the reactor and return false if done.
    pub fn step_reactor(&mut self) -> bool {
        // Activate the first matching module.
        if let Some(idx) = self.next_matching_module() {
            let slot = &mut self.reactor[idx];

            slot.status = ModuleStatus::SlotActive;
            if slot.condition.is_empty() {
                self.chain = 0.0;
            }
            self.chain += 1.0;
            slot.heat += self.chain;
            self.flux = self.flux.max(self.chain);
            self.action_queue.push_back(idx);
            self.last_action = slot.effect.clone();
            self.last_touched_idx = Some(idx);

            true
        } else {
            self.last_action.clear();
            self.last_touched_idx = None;
            false
        }
    }

    /// Determine whether the player is done attacking.
    pub fn is_player_done(&self) -> bool {
        self.action_queue.is_empty()
    }

    /// Take one step through the player's attack, returning the action or `None` if done.
    pub fn step_player(&mut self) -> Option<String> {
        self.last_touched_idx = self.action_queue.pop_front();
        if let Some(idx) = self.last_touched_idx {
            // Deactivate the reactor module and return its action.
            self.reactor[idx].status = if self.reactor[idx].heat > self.heat_capacity {
                ModuleStatus::SlotOverheated
            } else {
                ModuleStatus::SlotInactive
            };
            Some(self.reactor[idx].effect.clone())
        } else {
            // Action queue is done, so reset chain and flux.
            self.chain = 0.0;
            self.flux = 0.0;

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
    }

    /// Determine whether setting up the helm is done.
    pub fn is_setup_done(&self) -> bool {
        self.storage.is_empty() || self.hand.len() >= self.hand_size
    }

    /// Steps setting up the helm, returning false if setup was already complete.
    pub fn step_setup(&mut self, rng: &mut impl Rng) -> bool {
        if !self.storage.is_empty() && self.hand.len() < self.hand_size {
            self.draw_random(rng);
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
    pub actions: Vec<String>,
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
    pub fn step(&mut self, round: usize) -> Option<String> {
        if self.is_done(round) {
            self.action_idx = 0;
            self.flux = 0.0;
            None
        } else {
            self.action_idx += 1;
            self.flux += 1.0;
            Some(self.actions[self.action_idx - 1].clone())
        }
    }
}
