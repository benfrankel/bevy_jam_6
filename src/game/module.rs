use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ModuleAssets, Module)>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ModuleAssets {
    #[asset(path = "image/module/face_up.png")]
    bg_face_up: Handle<Image>,
    #[asset(path = "image/module/face_down.png")]
    bg_face_down: Handle<Image>,
    #[asset(path = "image/module/slot_empty.png")]
    bg_slot_empty: Handle<Image>,
    #[asset(path = "image/module/slot_inactive.png")]
    bg_slot_inactive: Handle<Image>,
    #[asset(path = "image/module/slot_active.png")]
    bg_slot_active: Handle<Image>,

    #[asset(path = "image/module/icon/nothing_condition.png")]
    nothing_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/nothing_effect.png")]
    nothing_effect_icon: Handle<Image>,
    #[asset(path = "image/module/icon/missile_condition.png")]
    missile_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/missile_effect.png")]
    missile_effect_icon: Handle<Image>,
    #[asset(path = "image/module/icon/laser_condition.png")]
    laser_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/laser_effect.png")]
    laser_effect_icon: Handle<Image>,
    #[asset(path = "image/module/icon/fire_condition.png")]
    fire_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/fire_effect.png")]
    fire_effect_icon: Handle<Image>,
    #[asset(path = "image/module/icon/heal_condition.png")]
    heal_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/heal_effect.png")]
    heal_effect_icon: Handle<Image>,
}

impl Configure for ModuleAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
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
    module_assets: Res<ModuleAssets>,
    module_query: Query<(Entity, &Module, &Children), Changed<Module>>,
    condition_query: Query<(), With<IsModuleConditionIcon>>,
    effect_query: Query<(), With<IsModuleEffectIcon>>,
    mut image_query: Query<&mut ImageNode>,
) {
    for (entity, module, children) in &module_query {
        // Update background image.
        c!(image_query.get_mut(entity)).image = match module.status {
            ModuleStatus::FaceUp => &module_assets.bg_face_up,
            ModuleStatus::FaceDown => &module_assets.bg_face_down,
            ModuleStatus::SlotEmpty => &module_assets.bg_slot_empty,
            ModuleStatus::SlotInactive => &module_assets.bg_slot_inactive,
            ModuleStatus::SlotActive => &module_assets.bg_slot_active,
        }
        .clone();

        // Update condition and effect icons.
        for &child in children {
            if condition_query.contains(child) {
                c!(image_query.get_mut(child)).image = match (&module.status, &module.condition) {
                    (ModuleStatus::FaceDown | ModuleStatus::SlotEmpty, _)
                    | (_, Action::Nothing) => &module_assets.nothing_condition_icon,
                    (_, Action::Missile) => &module_assets.missile_condition_icon,
                    (_, Action::Laser) => &module_assets.laser_condition_icon,
                    (_, Action::Fire) => &module_assets.fire_condition_icon,
                    (_, Action::Heal) => &module_assets.heal_condition_icon,
                }
                .clone();
            } else if effect_query.contains(child) {
                c!(image_query.get_mut(child)).image = match (&module.status, &module.effect) {
                    (ModuleStatus::FaceDown | ModuleStatus::SlotEmpty, _)
                    | (_, Action::Nothing) => &module_assets.nothing_effect_icon,
                    (_, Action::Missile) => &module_assets.missile_effect_icon,
                    (_, Action::Laser) => &module_assets.laser_effect_icon,
                    (_, Action::Fire) => &module_assets.fire_effect_icon,
                    (_, Action::Heal) => &module_assets.heal_effect_icon,
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
