pub mod helm;
pub mod module;
pub mod reactor;

use crate::animation::shake::NodeShake;
use crate::animation::shake::Shake;
use crate::animation::shake::ShakeRotation;
use crate::hud::helm::storage::StorageDisplay;
use crate::hud::reactor::ReactorIndex;
use crate::hud::reactor::flux_display::FluxLabel;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;
use crate::util::math::ExponentialFit;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<HudConfig>, Hud)>();

    app.add_plugins((helm::plugin, module::plugin, reactor::plugin));
}

pub fn hud(hud_config: &HudConfig, game_assets: &GameplayAssets) -> impl Bundle {
    (
        Name::new("Hud"),
        Hud,
        Node::ROW.full_size().abs(),
        hud_config.hud_shake,
        hud_config.hud_shake_rotation,
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
    flux_label_flux_trauma: ExponentialFit,

    module_shake: NodeShake,
    module_flux_trauma: ExponentialFit,

    pub camera_shake: Shake,
    pub camera_shake_rotation: ShakeRotation,
    pub camera_player_damage_trauma: ExponentialFit,

    pub hud_shake: NodeShake,
    pub hud_shake_rotation: ShakeRotation,
    pub hud_player_damage_trauma: ExponentialFit,
}

impl Config for HudConfig {
    const FILE: &'static str = "hud.ron";

    fn on_load(&mut self, world: &mut World) {
        for mut shake in world
            .query_filtered::<&mut NodeShake, With<FluxLabel>>()
            .iter_mut(world)
        {
            *shake = self.flux_label_shake;
        }

        for mut shake in world
            .query_filtered::<&mut NodeShake, Or<(With<ReactorIndex>, With<StorageDisplay>)>>()
            .iter_mut(world)
        {
            *shake = self.module_shake;
        }

        for (mut shake, mut shake_rotation) in world
            .query_filtered::<(&mut Shake, &mut ShakeRotation), With<IsDefaultUiCamera>>()
            .iter_mut(world)
        {
            *shake = self.camera_shake;
            *shake_rotation = self.camera_shake_rotation;
        }

        for (mut shake, mut shake_rotation) in world
            .query_filtered::<(&mut NodeShake, &mut ShakeRotation), With<Hud>>()
            .iter_mut(world)
        {
            *shake = self.hud_shake;
            *shake_rotation = self.hud_shake_rotation;
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Hud;

impl Configure for Hud {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
