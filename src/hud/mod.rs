pub mod helm;
mod module;
mod reactor;

use crate::animation::shake::NodeShake;
use crate::animation::shake::Shake;
use crate::animation::shake::ShakeWithCamera;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;
use crate::util::math::ScalingTrauma;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<HudConfig>>();

    app.add_plugins((helm::plugin, module::plugin, reactor::plugin));
}

pub fn hud(hud_config: &HudConfig, game_assets: &GameplayAssets) -> impl Bundle {
    (
        Name::new("Hud"),
        Node::ROW.full_size().abs(),
        ShakeWithCamera,
        NodeShake::default(),
        children![
            reactor::reactor(hud_config, game_assets),
            (
                Name::new("Column"),
                Node::COLUMN.reverse().grow(),
                children![helm::helm(hud_config, game_assets)],
            )
        ],
    )
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct HudConfig {
    storage_summary_actions: Vec<String>,

    flux_label_shake: NodeShake,
    flux_label_flux_trauma: ScalingTrauma,

    module_shake: NodeShake,
    module_flux_trauma: ScalingTrauma,

    pub camera_shake: Shake,
    pub camera_damage_trauma: ScalingTrauma,
    pub camera_ui_shake: NodeShake,
    pub camera_ui_damage_trauma: ScalingTrauma,
}

impl Config for HudConfig {
    const FILE: &'static str = "hud.ron";
}
