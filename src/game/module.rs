use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ModuleAssets>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ModuleAssets {
    #[asset(path = "image/module/face_up.png")]
    face_up_image: Handle<Image>,
    #[asset(path = "image/module/face_down.png")]
    face_down_image: Handle<Image>,
    #[asset(path = "image/module/slot_empty.png")]
    slot_empty_image: Handle<Image>,
    #[asset(path = "image/module/slot_inactive.png")]
    slot_inactive_image: Handle<Image>,
    #[asset(path = "image/module/slot_active.png")]
    slot_active_image: Handle<Image>,

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
    #[asset(path = "image/module/icon/heal_condition.png")]
    heal_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/heal_effect.png")]
    heal_effect_icon: Handle<Image>,
    #[asset(path = "image/module/icon/fire_condition.png")]
    fire_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/fire_effect.png")]
    fire_effect_icon: Handle<Image>,
}

impl Configure for ModuleAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}
