use crate::game::hud::HudAssets;
use crate::game::module::Module;
use crate::game::module::ModuleAction;
use crate::game::module::ModuleStatus;
use crate::prelude::*;

pub fn module(hud_assets: &HudAssets, module: Module, tooltip_anchor: Anchor) -> impl Bundle {
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

    let tooltip_text = match module.status {
        ModuleStatus::FaceDown => "[b]Reactor module[r]".to_string(),
        ModuleStatus::SlotEmpty => "[b]Reactor module[r]\n\nEmpty slot".to_string(),
        _ => {
            let condition = match module.condition {
                ModuleAction::Nothing => "Unconditionally ",
                ModuleAction::Missile => "After firing a missile, ",
                ModuleAction::Laser => "After firing a laser, ",
                ModuleAction::Fire => "After breathing fire, ",
                ModuleAction::Heal => "After repairing the hull, ",
            };
            let effect = match (&module.condition, &module.effect) {
                (_, ModuleAction::Nothing) => "do nothing",
                (ModuleAction::Missile, ModuleAction::Missile) => "fire another missile",
                (_, ModuleAction::Missile) => "fire a missile",
                (ModuleAction::Laser, ModuleAction::Laser) => "fire another laser",
                (_, ModuleAction::Laser) => "fire a laser",
                (ModuleAction::Fire, ModuleAction::Fire) => "breathe more fire",
                (_, ModuleAction::Fire) => "breathe fire",
                (ModuleAction::Heal, ModuleAction::Heal) => "repair the hull again",
                (_, ModuleAction::Heal) => "repair the hull",
            };
            format!("[b]Reactor module[r]\n\n{condition}{effect}.")
        },
    };

    (
        Name::new("Module"),
        ImageNode::from(background),
        Node {
            width: Vw(6.66),
            aspect_ratio: Some(1.0),
            ..Node::ROW_CENTER
        },
        Tooltip::fixed(tooltip_anchor, parse_rich(tooltip_text)),
        children![
            (
                Name::new("Condition"),
                ImageNode::from(condition_icon),
                Node::default().full_size().abs(),
            ),
            (
                Name::new("Effect"),
                ImageNode::from(effect_icon),
                Node::default().full_size().abs(),
            ),
        ],
    )
}
