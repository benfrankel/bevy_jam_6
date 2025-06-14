use crate::game::GameAssets;
use crate::game::module::Module;
use crate::game::module::ModuleAction;
use crate::game::module::ModuleStatus;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ModuleSlotGlow>();
}

pub fn module(game_assets: &GameAssets, module: Module, heat_capacity: f32) -> impl Bundle {
    let background = match module.status {
        ModuleStatus::FaceUp => &game_assets.module_face_up,
        ModuleStatus::FaceDown => &game_assets.module_face_down,
        ModuleStatus::SlotEmpty => &game_assets.module_slot_empty,
        ModuleStatus::SlotInactive => &game_assets.module_slot_inactive,
        ModuleStatus::SlotActive => &game_assets.module_slot_active,
        ModuleStatus::SlotOverheated => &game_assets.module_slot_overheated,
    }
    .clone();

    let condition_icon = match (&module.status, &module.condition) {
        (ModuleStatus::FaceDown | ModuleStatus::SlotEmpty, _) | (_, ModuleAction::Nothing) => {
            &game_assets.nothing_condition_icon
        },
        (_, ModuleAction::Missile) => &game_assets.missile_condition_icon,
        (_, ModuleAction::Laser) => &game_assets.laser_condition_icon,
        (_, ModuleAction::Fireball) => &game_assets.fireball_condition_icon,
        (_, ModuleAction::Repair) => &game_assets.repair_condition_icon,
    }
    .clone();

    let effect_icon = match (&module.status, &module.effect) {
        (ModuleStatus::FaceDown | ModuleStatus::SlotEmpty, _) | (_, ModuleAction::Nothing) => {
            &game_assets.nothing_effect_icon
        },
        (_, ModuleAction::Missile) => &game_assets.missile_effect_icon,
        (_, ModuleAction::Laser) => &game_assets.laser_effect_icon,
        (_, ModuleAction::Fireball) => &game_assets.fireball_effect_icon,
        (_, ModuleAction::Repair) => &game_assets.repair_effect_icon,
    }
    .clone();

    let heat = match module.status {
        ModuleStatus::SlotOverheated => 1.0,
        ModuleStatus::SlotInactive => (module.heat / heat_capacity.max(1.0)).clamp(0.0, 1.0),
        _ => 0.0,
    };
    let glow = if matches!(module.status, ModuleStatus::SlotOverheated) {
        &game_assets.module_slot_full_glow
    } else {
        &game_assets.module_slot_glow
    }
    .clone();

    (
        Name::new("Module"),
        ImageNode::from(background),
        Node {
            width: Vw(6.6666),
            aspect_ratio: Some(1.0),
            ..Node::ROW_CENTER
        },
        children![
            (
                Name::new("Condition"),
                ImageNode::from(condition_icon),
                Node::default().full_size().abs(),
                Pickable::IGNORE,
            ),
            (
                Name::new("Effect"),
                ImageNode::from(effect_icon),
                Node::default().full_size().abs(),
                Pickable::IGNORE,
            ),
            (
                Name::new("Glow"),
                ModuleSlotGlow,
                ImageNode::from(glow).with_color(Color::srgb(0.831, 0.463, 0.459).with_alpha(heat)),
                Node::default().full_size().abs(),
                ZIndex(1),
                Pickable::IGNORE,
            ),
        ],
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ModuleSlotGlow;

impl Configure for ModuleSlotGlow {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
