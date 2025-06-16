use crate::module::Module;
use crate::module::ModuleConfig;
use crate::module::ModuleStatus;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ModuleSlotGlow>();
}

pub fn module(
    game_assets: &GameplayAssets,
    module_config: &ModuleConfig,
    module: &Module,
    heat_capacity: f32,
) -> impl Bundle {
    let background = match module.status {
        ModuleStatus::FaceUp => &game_assets.module_face_up,
        ModuleStatus::FaceDown => &game_assets.module_face_down,
        ModuleStatus::SlotEmpty => &game_assets.module_slot_empty,
        ModuleStatus::SlotInactive => &game_assets.module_slot_inactive,
        ModuleStatus::SlotActive => &game_assets.module_slot_active,
        ModuleStatus::SlotOverheated => &game_assets.module_slot_overheated,
    }
    .clone();

    let condition = &module_config.actions[&module.condition];
    let effect = &module_config.actions[&module.effect];
    let show_icons = if matches!(
        module.status,
        ModuleStatus::FaceDown | ModuleStatus::SlotEmpty,
    ) {
        0.0
    } else {
        1.0
    };

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
            width: Vw(6.66666),
            aspect_ratio: Some(1.0),
            ..Node::ROW_CENTER
        },
        children![
            (
                Name::new("Condition"),
                ImageNode::from(condition.condition_icon.clone())
                    .with_color(Color::WHITE.with_alpha(show_icons)),
                Node::default().full_size().abs(),
                Pickable::IGNORE,
            ),
            (
                Name::new("Effect"),
                ImageNode::from(effect.effect_icon.clone())
                    .with_color(Color::WHITE.with_alpha(show_icons)),
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
