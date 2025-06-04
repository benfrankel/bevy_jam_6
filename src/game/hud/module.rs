use crate::game::hud::HudAssets;
use crate::game::module::Module;
use crate::game::module::ModuleAction;
use crate::game::module::ModuleStatus;
use crate::prelude::*;

pub fn module(hud_assets: &HudAssets, module: Module) -> impl Bundle {
    let background = match module.status {
        ModuleStatus::FaceUp => &hud_assets.module_face_up,
        ModuleStatus::FaceDown => &hud_assets.module_face_down,
        ModuleStatus::SlotEmpty => &hud_assets.module_slot_empty,
        ModuleStatus::SlotInactive => &hud_assets.module_slot_inactive,
        ModuleStatus::SlotActive => &hud_assets.module_slot_active,
    }
    .clone();

    let condition_icon = match (&module.status, &module.condition) {
        (ModuleStatus::FaceDown | ModuleStatus::SlotEmpty, _) | (_, ModuleAction::Nothing) => {
            &hud_assets.nothing_condition_icon
        },
        (_, ModuleAction::Missile) => &hud_assets.missile_condition_icon,
        (_, ModuleAction::Laser) => &hud_assets.laser_condition_icon,
        (_, ModuleAction::Fire) => &hud_assets.fire_condition_icon,
        (_, ModuleAction::Heal) => &hud_assets.heal_condition_icon,
    }
    .clone();

    let effect_icon = match (&module.status, &module.effect) {
        (ModuleStatus::FaceDown | ModuleStatus::SlotEmpty, _) | (_, ModuleAction::Nothing) => {
            &hud_assets.nothing_effect_icon
        },
        (_, ModuleAction::Missile) => &hud_assets.missile_effect_icon,
        (_, ModuleAction::Laser) => &hud_assets.laser_effect_icon,
        (_, ModuleAction::Fire) => &hud_assets.fire_effect_icon,
        (_, ModuleAction::Heal) => &hud_assets.heal_effect_icon,
    }
    .clone();

    (
        Name::new("Module"),
        ImageNode::from(background),
        Node {
            width: Vw(6.66),
            aspect_ratio: Some(1.0),
            ..Node::ROW_CENTER
        },
        Pickable::IGNORE,
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
        ],
    )
}
