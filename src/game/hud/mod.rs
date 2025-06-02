mod flux;
mod module;
mod reactor;
mod stage;

use crate::game::hud::reactor::reactor;
use crate::game::hud::stage::stage;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<HudAssets>();

    app.add_plugins((flux::plugin, module::plugin, reactor::plugin, stage::plugin));
}

pub fn hud(hud_assets: &HudAssets) -> impl Bundle {
    (
        Name::new("Hud"),
        Node::ROW.full_size().abs(),
        children![
            reactor(hud_assets),
            (
                Name::new("Column"),
                Node {
                    flex_grow: 1.0,
                    ..Node::COLUMN.reverse()
                },
                children![stage(hud_assets)],
            )
        ],
    )
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct HudAssets {
    // Layout background images.
    #[asset(path = "image/ui/reactor.png")]
    reactor: Handle<Image>,
    #[asset(path = "image/ui/stage.png")]
    stage: Handle<Image>,

    // Reactor module background images.
    #[asset(path = "image/module/face_up.png")]
    module_face_up: Handle<Image>,
    #[asset(path = "image/module/face_down.png")]
    module_face_down: Handle<Image>,
    #[asset(path = "image/module/slot_empty.png")]
    module_slot_empty: Handle<Image>,
    #[asset(path = "image/module/slot_inactive.png")]
    module_slot_inactive: Handle<Image>,
    #[asset(path = "image/module/slot_active.png")]
    module_slot_active: Handle<Image>,

    // Reactor module icons.
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

impl Configure for HudAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}
