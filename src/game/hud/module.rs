use crate::game::hud::HudAssets;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Module>();
}

pub fn module(module: Module, tooltip_anchor: Anchor) -> impl Bundle {
    (
        Name::new("Module"),
        module,
        ImageNode::default(),
        Node {
            width: Vw(6.66),
            aspect_ratio: Some(1.0),
            ..Node::ROW_CENTER
        },
        Tooltip::fixed(tooltip_anchor, ""),
        children![
            (
                Name::new("Condition"),
                ImageNode::default(),
                Node::default().full_size().abs(),
                IsModuleConditionIcon,
            ),
            (
                Name::new("Effect"),
                ImageNode::default(),
                Node::default().full_size().abs(),
                IsModuleEffectIcon,
            ),
        ],
    )
}

#[derive(Reflect, Debug, Copy, Clone, Default)]
pub enum Action {
    #[default]
    Nothing,
    Missile,
    Laser,
    Fire,
    Heal,
}

#[derive(Reflect, Debug, Copy, Clone, Default)]
pub enum ModuleStatus {
    #[default]
    FaceUp,
    FaceDown,
    SlotEmpty,
    SlotInactive,
    SlotActive,
}

#[derive(Component, Reflect, Debug, Copy, Clone, Default)]
#[reflect(Component)]
pub struct Module {
    pub condition: Action,
    pub effect: Action,
    pub status: ModuleStatus,
}

impl Configure for Module {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.configure::<(IsModuleConditionIcon, IsModuleEffectIcon)>();
        app.add_systems(
            Update,
            (
                sync_module_tooltips.in_set(UpdateSystems::SyncLate),
                sync_module_images.in_set(UpdateSystems::SyncLate),
            ),
        );
    }
}

impl Module {
    pub const EMPTY: Self = Self {
        condition: Action::Nothing,
        effect: Action::Nothing,
        status: ModuleStatus::SlotEmpty,
    };
}

fn sync_module_tooltips(mut module_query: Query<(&mut Tooltip, &Module), Changed<Module>>) {
    for (mut tooltip, module) in &mut module_query {
        let text = match module.status {
            ModuleStatus::FaceDown => "[b]Reactor module[r]".to_string(),
            ModuleStatus::SlotEmpty => "[b]Reactor module[r]\n\nEmpty slot".to_string(),
            _ => {
                let condition = match module.condition {
                    Action::Nothing => "Unconditionally ",
                    Action::Missile => "After firing a missile, ",
                    Action::Laser => "After firing a laser, ",
                    Action::Fire => "After breathing fire, ",
                    Action::Heal => "After repairing the hull, ",
                };
                let effect = match (&module.condition, &module.effect) {
                    (_, Action::Nothing) => "do nothing",
                    (Action::Missile, Action::Missile) => "fire another missile",
                    (_, Action::Missile) => "fire a missile",
                    (Action::Laser, Action::Laser) => "fire another laser",
                    (_, Action::Laser) => "fire a laser",
                    (Action::Fire, Action::Fire) => "breathe more fire",
                    (_, Action::Fire) => "breathe fire",
                    (Action::Heal, Action::Heal) => "repair the hull again",
                    (_, Action::Heal) => "repair the hull",
                };
                format!("[b]Reactor module[r]\n\n{condition}{effect}.")
            },
        };
        tooltip.content = TooltipContent::Primary(RichText::from_sections(parse_rich(text)));
    }
}

fn sync_module_images(
    hud_assets: Res<HudAssets>,
    module_query: Query<(Entity, &Module, &Children), Changed<Module>>,
    condition_query: Query<(), With<IsModuleConditionIcon>>,
    effect_query: Query<(), With<IsModuleEffectIcon>>,
    mut image_query: Query<&mut ImageNode>,
) {
    for (entity, module, children) in &module_query {
        // Update background image.
        c!(image_query.get_mut(entity)).image = match module.status {
            ModuleStatus::FaceUp => &hud_assets.module_face_up,
            ModuleStatus::FaceDown => &hud_assets.module_face_down,
            ModuleStatus::SlotEmpty => &hud_assets.module_slot_empty,
            ModuleStatus::SlotInactive => &hud_assets.module_slot_inactive,
            ModuleStatus::SlotActive => &hud_assets.module_slot_active,
        }
        .clone();

        // Update condition and effect icons.
        for &child in children {
            if condition_query.contains(child) {
                c!(image_query.get_mut(child)).image = match (&module.status, &module.condition) {
                    (ModuleStatus::FaceDown | ModuleStatus::SlotEmpty, _)
                    | (_, Action::Nothing) => &hud_assets.nothing_condition_icon,
                    (_, Action::Missile) => &hud_assets.missile_condition_icon,
                    (_, Action::Laser) => &hud_assets.laser_condition_icon,
                    (_, Action::Fire) => &hud_assets.fire_condition_icon,
                    (_, Action::Heal) => &hud_assets.heal_condition_icon,
                }
                .clone();
            } else if effect_query.contains(child) {
                c!(image_query.get_mut(child)).image = match (&module.status, &module.effect) {
                    (ModuleStatus::FaceDown | ModuleStatus::SlotEmpty, _)
                    | (_, Action::Nothing) => &hud_assets.nothing_effect_icon,
                    (_, Action::Missile) => &hud_assets.missile_effect_icon,
                    (_, Action::Laser) => &hud_assets.laser_effect_icon,
                    (_, Action::Fire) => &hud_assets.fire_effect_icon,
                    (_, Action::Heal) => &hud_assets.heal_effect_icon,
                }
                .clone();
            }
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsModuleConditionIcon;

impl Configure for IsModuleConditionIcon {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsModuleEffectIcon;

impl Configure for IsModuleEffectIcon {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
