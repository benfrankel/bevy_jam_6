mod flux;
pub mod helm;
mod module;
mod reactor;

use crate::game::hud::helm::helm;
use crate::game::hud::reactor::reactor;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<HudConfig>, HudAssets)>();

    app.add_plugins((flux::plugin, reactor::plugin, helm::plugin));
}

pub fn hud(hud_assets: &HudAssets) -> impl Bundle {
    (
        Name::new("Hud"),
        Node::ROW.full_size().abs(),
        children![
            reactor(hud_assets),
            (
                Name::new("Column"),
                ..Node::COLUMN.reverse().grow(),
                children![helm(hud_assets)],
            )
        ],
    )
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct HudConfig {
    flux_shake_magnitude: Vec2,
    flux_shake_decay: f32,
}

impl Config for HudConfig {
    const FILE: &'static str = "hud.ron";
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct HudAssets {
    // Layout background images.
    #[asset(path = "image/ui/reactor.png")]
    reactor: Handle<Image>,
    #[asset(path = "image/ui/helm.png")]
    helm: Handle<Image>,

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
    #[asset(path = "image/module/slot_overheated.png")]
    module_slot_overheated: Handle<Image>,

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

    // Buttons.
    #[asset(path = "image/ui/info_button.png")]
    info_button: Handle<Image>,
    #[asset(path = "image/ui/pause_button.png")]
    pause_button: Handle<Image>,
    #[asset(path = "image/ui/skip_button.png")]
    skip_button: Handle<Image>,
}

impl Configure for HudAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}
