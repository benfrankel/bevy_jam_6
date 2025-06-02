use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Deck, DeckActions)>();
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
    pub fn reset(&mut self) {
        self.flux = 0.0;
        for slot in &mut self.reactor {
            *slot = Module::EMPTY;
        }
        self.storage.append(&mut self.hand);
        self.selected_idx = 0;
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

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum DeckActions {
    SelectLeft,
    SelectRight,
    Play,
}

impl Configure for DeckActions {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::SelectLeft, GamepadButton::DPadLeft)
                .with(Self::SelectLeft, GamepadButton::LeftTrigger)
                .with(Self::SelectLeft, KeyCode::KeyA)
                .with(Self::SelectLeft, KeyCode::ArrowLeft)
                .with(Self::SelectRight, GamepadButton::DPadRight)
                .with(Self::SelectRight, GamepadButton::RightTrigger)
                .with(Self::SelectRight, KeyCode::KeyD)
                .with(Self::SelectRight, KeyCode::ArrowRight)
                .with(Self::Play, GamepadButton::East)
                .with(Self::Play, KeyCode::Space)
                .with(Self::Play, KeyCode::Enter)
                .with(Self::Play, KeyCode::NumpadEnter),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            (
                select_left
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::SelectLeft)),
                select_right
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::SelectRight)),
                play.in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::Play)),
            ),
        );
    }
}

fn select_left(mut deck: ResMut<Deck>) {
    deck.selected_idx = deck.selected_idx.saturating_sub(1);
}

fn select_right(mut deck: ResMut<Deck>) {
    deck.selected_idx = (deck.selected_idx + 1).min(deck.hand.len().saturating_sub(1));
}

fn play(mut deck: ResMut<Deck>) {
    rq!(!deck.hand.is_empty());

    // Remove the selected module from hand.
    let idx = deck.selected_idx;
    let mut selected = deck.hand.remove(idx);

    // Place it in the next reactor slot.
    let idx = deck.next_slot;
    if !matches!(deck.reactor[idx].status, ModuleStatus::SlotEmpty) {
        let mut replaced = deck.reactor[idx];
        replaced.status = ModuleStatus::FaceUp;
        deck.storage.push(replaced);
    }
    selected.status = ModuleStatus::SlotInactive;
    deck.reactor[idx] = selected;
    deck.next_slot += 1;
    if deck.next_slot >= deck.reactor.len() {
        deck.next_slot = 0;
    }

    // Draw a new module to hand.
    if !deck.storage.is_empty() {
        let idx = thread_rng().gen_range(0..deck.storage.len());
        let draw = deck.storage.swap_remove(idx);
        deck.hand.push(draw);
    }

    // Clamp selected index within new hand size.
    deck.selected_idx = deck
        .selected_idx
        .clamp(0, deck.hand.len().saturating_sub(1));
}
