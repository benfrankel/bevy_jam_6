use crate::animation::offset::NodeOffset;
use crate::animation::shake::NodeShake;
use crate::game::hud::HudAssets;
use crate::game::module::Module;
use crate::game::module::ModuleAction;
use crate::game::module::ModuleStatus;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsModuleSlotGlow>();
}

pub fn module(
    hud_assets: &HudAssets,
    module: Module,
    heat_capacity: f32,
    shake: NodeShake,
) -> impl Bundle {
    let background = match module.status {
        ModuleStatus::FaceUp => &hud_assets.module_face_up,
        ModuleStatus::FaceDown => &hud_assets.module_face_down,
        ModuleStatus::SlotEmpty => &hud_assets.module_slot_empty,
        ModuleStatus::SlotInactive => &hud_assets.module_slot_inactive,
        ModuleStatus::SlotActive => &hud_assets.module_slot_active,
        ModuleStatus::SlotOverheated => &hud_assets.module_slot_overheated,
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

    let heat = if matches!(
        module.status,
        ModuleStatus::SlotInactive | ModuleStatus::SlotOverheated
    ) {
        (module.heat / heat_capacity.max(1.0)).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let glow = if matches!(module.status, ModuleStatus::SlotOverheated) {
        &hud_assets.module_slot_full_glow
    } else {
        &hud_assets.module_slot_glow
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
        NodeOffset::new(Vw(0.), Vw(0.)),
        shake,
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
            (
                Name::new("Glow"),
                IsModuleSlotGlow,
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
struct IsModuleSlotGlow;

impl Configure for IsModuleSlotGlow {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
