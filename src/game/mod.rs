pub mod combat;
pub mod deck;
pub mod hud;
pub mod level;
pub mod module;
pub mod phase;
pub mod projectile;
pub mod ship;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<GameAssets>();

    app.add_plugins((
        combat::plugin,
        deck::plugin,
        hud::plugin,
        level::plugin,
        projectile::plugin,
        module::plugin,
        ship::plugin,
        phase::plugin,
    ));
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct GameAssets {
    // Space background images.
    #[asset(path = "image/space/level0.png")]
    pub bg_level0: Handle<Image>,
    #[asset(path = "image/space/level1.png")]
    pub bg_level1: Handle<Image>,
    #[asset(path = "image/space/level2.png")]
    pub bg_level2: Handle<Image>,
    #[asset(path = "image/space/level3.png")]
    pub bg_level3: Handle<Image>,
    #[asset(path = "image/space/level4.png")]
    pub bg_level4: Handle<Image>,
    #[asset(path = "image/space/level5.png")]
    pub bg_level5: Handle<Image>,
    #[asset(path = "image/space/level6.png")]
    pub bg_level6: Handle<Image>,
    #[asset(path = "image/space/level7.png")]
    pub bg_level7: Handle<Image>,
    #[asset(path = "image/space/level8.png")]
    pub bg_level8: Handle<Image>,
    #[asset(path = "image/space/level9.png")]
    pub bg_level9: Handle<Image>,

    // HUD layout.
    #[asset(path = "image/ui/reactor.png")]
    pub reactor: Handle<Image>,
    #[asset(path = "image/ui/helm.png")]
    pub helm: Handle<Image>,

    // Reactor module background images and VFX.
    #[asset(path = "image/module/face_up.png")]
    pub module_face_up: Handle<Image>,
    #[asset(path = "image/module/face_down.png")]
    pub module_face_down: Handle<Image>,
    #[asset(path = "image/module/slot_empty.png")]
    pub module_slot_empty: Handle<Image>,
    #[asset(path = "image/module/slot_inactive.png")]
    pub module_slot_inactive: Handle<Image>,
    #[asset(path = "image/module/slot_active.png")]
    pub module_slot_active: Handle<Image>,
    #[asset(path = "image/module/slot_overheated.png")]
    pub module_slot_overheated: Handle<Image>,
    #[asset(path = "image/module/glow.png")]
    pub module_slot_glow: Handle<Image>,
    #[asset(path = "image/module/full_glow.png")]
    pub module_slot_full_glow: Handle<Image>,

    // Reactor module icons.
    #[asset(path = "image/module/icon/nothing_condition.png")]
    pub nothing_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/nothing_effect.png")]
    pub nothing_effect_icon: Handle<Image>,
    #[asset(path = "image/module/icon/missile_condition.png")]
    pub missile_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/missile_effect.png")]
    pub missile_effect_icon: Handle<Image>,
    #[asset(path = "image/module/icon/laser_condition.png")]
    pub laser_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/laser_effect.png")]
    pub laser_effect_icon: Handle<Image>,
    #[asset(path = "image/module/icon/fireball_condition.png")]
    pub fireball_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/fireball_effect.png")]
    pub fireball_effect_icon: Handle<Image>,
    #[asset(path = "image/module/icon/repair_condition.png")]
    pub repair_condition_icon: Handle<Image>,
    #[asset(path = "image/module/icon/repair_effect.png")]
    pub repair_effect_icon: Handle<Image>,

    // Phase display.
    #[asset(path = "image/ui/phase_setup.png")]
    pub phase_setup: Handle<Image>,
    #[asset(path = "image/ui/phase_player.png")]
    pub phase_player: Handle<Image>,
    #[asset(path = "image/ui/phase_reactor.png")]
    pub phase_reactor: Handle<Image>,
    #[asset(path = "image/ui/phase_enemy.png")]
    pub phase_enemy: Handle<Image>,

    // Buttons.
    #[asset(path = "image/ui/info_button.png")]
    pub info_button: Handle<Image>,
    #[asset(path = "image/ui/pause_button.png")]
    pub pause_button: Handle<Image>,
    #[asset(path = "image/ui/skip_button.png")]
    pub skip_button: Handle<Image>,

    // Spaceships.
    #[asset(path = "image/ship/player.png")]
    pub player_ship: Handle<Image>,
    #[asset(path = "image/ship/enemy.png")]
    pub enemy_ship: Handle<Image>,

    // Projectiles.
    #[asset(path = "image/projectile/missile.png")]
    pub missile: Handle<Image>,
    #[asset(path = "image/projectile/laser.png")]
    pub laser: Handle<Image>,
    #[asset(path = "image/projectile/fireball.png")]
    pub fireball: Handle<Image>,

    // VFX.
    #[asset(path = "image/vfx/heal_popup.png")]
    pub heal_popup: Handle<Image>,

    // Music.
    #[asset(path = "audio/music/545458__bertsz__bit-forest-evil-theme-music.ogg")]
    pub music: Handle<AudioSource>,

    // SFX.
    #[asset(path = "audio/sfx/Firing Laser SFX.ogg")]
    pub laser_spawn_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Fireball Hit SFX.ogg")]
    pub fireball_spawn_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Spaceship SFX 1.ogg")]
    pub repair_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Reactor Module SFX_2 Base.ogg")]
    pub module_activate_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Click SFX 1.ogg")]
    pub module_insert_sfx: Handle<AudioSource>,
}

impl Configure for GameAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}
