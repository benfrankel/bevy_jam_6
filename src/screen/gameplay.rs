use crate::game::level::Level;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        (
            Screen::Gameplay.on_edge(Level::disable, (Level(0).enter(), Level::trigger).chain()),
            Menu::ANY
                .on_enable(spawn_menu_overlay)
                .run_if(Screen::Gameplay.will_update()),
        ),
    );

    app.configure::<GameplayAction>();
}

fn spawn_menu_overlay(mut commands: Commands) {
    commands.spawn((
        widget::blocking_overlay(1),
        ThemeColor::Overlay.set::<BackgroundColor>(),
        DespawnOnExitState::<Screen>::default(),
        DespawnOnDisableState::<Menu>::default(),
    ));
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameplayAction {
    Pause,
    CloseMenu,
    ToggleTooltips,
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
                .with(Self::ToggleTooltips, KeyCode::KeyI),
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
                toggle_tooltips
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_just_pressed(Self::ToggleTooltips)),
            )),
        );
    }
}

fn toggle_tooltips(mut tooltip_settings: ResMut<TooltipSettings>) {
    tooltip_settings.enabled ^= true;
}
