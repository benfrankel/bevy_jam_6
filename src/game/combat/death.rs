use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::game::GameAssets;
use crate::game::level::Level;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<OnDeath>();
}

#[derive(Event, Reflect, Debug)]
pub struct OnDeath;

impl Configure for OnDeath {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(despawn_on_death);
        app.add_observer(play_ship_death_sfx_on_death);
    }
}

fn despawn_on_death(trigger: Trigger<OnDeath>, mut commands: Commands) {
    commands.entity(r!(trigger.get_target())).try_despawn();
}

fn play_ship_death_sfx_on_death(
    _: Trigger<OnDeath>,
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    game_assets: Res<GameAssets>,
) {
    commands.spawn((
        sfx_audio(&audio_settings, game_assets.ship_death_sfx.clone(), 1.0),
        DespawnOnExitState::<Level>::default(),
    ));
}
