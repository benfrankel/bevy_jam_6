pub mod helm;
mod module;
mod reactor;

use crate::animation::shake::NodeShake;
use crate::animation::shake::ShakeWithScreen;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<HudConfig>>();

    app.add_plugins((helm::plugin, module::plugin, reactor::plugin));
}

pub fn hud(game_assets: &GameplayAssets) -> impl Bundle {
    (
        Name::new("Hud"),
        Node::ROW.full_size().abs(),
        ShakeWithScreen,
        NodeShake::default(),
        children![
            reactor::reactor(game_assets),
            (
                Name::new("Column"),
                Node::COLUMN.reverse().grow(),
                children![helm::helm(game_assets)],
            )
        ],
    )
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct HudConfig {
    storage_summary_actions: Vec<String>,

    flux_shake_flux_factor: f32,
    flux_shake_flux_min: f32,
    flux_shake_amplitude: Vec2,
    flux_shake_trauma: f32,
    flux_shake_decay: f32,
    flux_shake_exponent: f32,

    pub module_shake_flux_factor: f32,
    pub module_shake_flux_min: f32,
    pub module_shake_amplitude: Vec2,
    pub module_shake_trauma: f32,
    pub module_shake_decay: f32,
    pub module_shake_exponent: f32,

    pub screen_shake_damage_factor: f32,
    pub screen_shake_damage_min: f32,
    pub ui_screen_shake_amplitude: Vec2,
    pub ui_screen_shake_trauma: f32,
    pub ui_screen_shake_decay: f32,
    pub ui_screen_shake_exponent: f32,
    pub camera_screen_shake_amplitude: Vec2,
    pub camera_screen_shake_trauma: f32,
    pub camera_screen_shake_decay: f32,
    pub camera_screen_shake_exponent: f32,

    pub player_ship_shake_damage_factor: f32,
    pub player_ship_shake_damage_min: f32,
    pub player_ship_shake_amplitude: Vec2,
    pub player_ship_shake_trauma: f32,
    pub player_ship_shake_decay: f32,
    pub player_ship_shake_exponent: f32,

    pub enemy_ship_shake_damage_factor: f32,
    pub enemy_ship_shake_damage_min: f32,
    pub enemy_ship_shake_amplitude: Vec2,
    pub enemy_ship_shake_trauma: f32,
    pub enemy_ship_shake_decay: f32,
    pub enemy_ship_shake_exponent: f32,
}

impl Config for HudConfig {
    const FILE: &'static str = "hud.ron";
}
