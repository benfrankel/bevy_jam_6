use crate::level::Level;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        (
            Screen::Gameplay.on_edge(Level::disable, (Level(0).enter(), Level::trigger).chain()),
            Menu::ANY.on_enter(
                spawn_menu_overlay.run_if(
                    Screen::Gameplay
                        .will_enter()
                        .and(Screen::is_triggered.or(Menu::is_disabled)),
                ),
            ),
        ),
    );

    app.configure::<(GameplayAssets, GameplayAction)>();
}

fn spawn_menu_overlay(mut commands: Commands) {
    commands.spawn((
        widget::blocking_overlay(1),
        ThemeColor::Overlay.set::<BackgroundColor>(),
        DespawnOnExitState::<Screen>::default(),
        DespawnOnDisableState::<Menu>::default(),
    ));
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct GameplayAssets {
    // Space background images.
    #[asset(path = "image/space/space0.png")]
    pub bg_level0: Handle<Image>,
    #[asset(path = "image/space/space1.png")]
    pub bg_level1: Handle<Image>,
    #[asset(path = "image/space/space2.png")]
    pub bg_level2: Handle<Image>,
    #[asset(path = "image/space/space3.png")]
    pub bg_level3: Handle<Image>,
    #[asset(path = "image/space/space4.png")]
    pub bg_level4: Handle<Image>,
    #[asset(path = "image/space/space5.png")]
    pub bg_level5: Handle<Image>,
    #[asset(path = "image/space/space6.png")]
    pub bg_level6: Handle<Image>,
    #[asset(path = "image/space/space7.png")]
    pub bg_level7: Handle<Image>,
    #[asset(path = "image/space/space8.png")]
    pub bg_level8: Handle<Image>,
    #[asset(path = "image/space/space9.png")]
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

    // Upgrade icons.
    #[asset(path = "image/ui/upgrade_pack_nothing.png")]
    pub upgrade_pack_nothing: Handle<Image>,
    #[asset(path = "image/ui/upgrade_pack_repair.png")]
    pub upgrade_pack_repair: Handle<Image>,
    #[asset(path = "image/ui/upgrade_pack_missile.png")]
    pub upgrade_pack_missile: Handle<Image>,
    #[asset(path = "image/ui/upgrade_pack_laser.png")]
    pub upgrade_pack_laser: Handle<Image>,
    #[asset(path = "image/ui/upgrade_pack_fireball.png")]
    pub upgrade_pack_fireball: Handle<Image>,
    #[asset(path = "image/ui/upgrade_capacitor.png")]
    pub upgrade_capacitor: Handle<Image>,
    #[asset(path = "image/ui/upgrade_cooler.png")]
    pub upgrade_cooler: Handle<Image>,
    #[asset(path = "image/ui/upgrade_alloy.png")]
    pub upgrade_alloy: Handle<Image>,

    // Spaceships.
    #[asset(path = "image/ship/player.png")]
    pub player_ship: Handle<Image>,
    #[asset(path = "image/ship/enemy.png")]
    pub enemy_ship: Handle<Image>,

    // VFX.
    #[asset(path = "image/vfx/heal_popup.png")]
    pub heal_popup: Handle<Image>,

    // SFX.
    #[asset(path = "audio/sfx/Movement SFX 3.ogg")]
    pub module_hover_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Click SFX 1.ogg")]
    pub module_insert_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Reactor Module SFX_3 Base.ogg")]
    pub module_activate_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Deactivating Module SFX.ogg")]
    pub phase_change_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Metal Hit SFX 1.ogg")]
    pub ship_hurt_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Ship Destroyed SFX.ogg")]
    pub ship_death_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Spaceship SFX 1.ogg")]
    pub repair_sfx: Handle<AudioSource>,
}

impl Configure for GameplayAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameplayAction {
    Pause,
    CloseMenu,
    ToggleHelp,
}

impl Configure for GameplayAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::Pause, GamepadButton::Start)
                .with(Self::Pause, KeyCode::Escape)
                .with(Self::Pause, KeyCode::KeyP)
                .with(Self::CloseMenu, KeyCode::KeyP)
                .with(Self::ToggleHelp, KeyCode::KeyI),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            Screen::Gameplay.on_update((
                Menu::Pause
                    .enter()
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_disabled.and(action_just_pressed(Self::Pause))),
                Menu::clear
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_enabled.and(action_just_pressed(Self::CloseMenu))),
                Menu::Help
                    .enter()
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_disabled.and(action_just_pressed(Self::ToggleHelp))),
                Menu::pop.in_set(UpdateSystems::RecordInput).run_if(
                    Menu::Help
                        .will_update()
                        .and(action_just_pressed(Self::ToggleHelp)),
                ),
            )),
        );
    }
}
