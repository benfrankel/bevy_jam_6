use bevy::audio::AudioPlugin;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<AudioSettings>();

    app.add_plugins(AudioPlugin::default());
}

#[derive(Resource, Reflect, Clone, Debug)]
#[reflect(Resource)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub ui_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.5,
            music_volume: 0.5,
            sfx_volume: 0.5,
            ui_volume: 0.5,
        }
    }
}

impl Configure for AudioSettings {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.configure::<(MusicAudio, SfxAudio, UiAudio)>();
        app.add_systems(
            Update,
            apply_audio_settings
                .run_if(resource_changed::<Self>)
                .in_set(UpdateSystems::Update),
        );
    }
}

impl AudioSettings {
    pub fn master_volume(&self) -> Volume {
        position_to_volume(self.master_volume)
    }

    pub fn music_volume(&self) -> Volume {
        self.master_volume() * position_to_volume(self.music_volume)
    }

    pub fn sfx_volume(&self) -> Volume {
        self.master_volume() * position_to_volume(self.sfx_volume)
    }

    pub fn ui_volume(&self) -> Volume {
        self.master_volume() * position_to_volume(self.ui_volume)
    }
}

/// Map a volume selector position (in the [0, 1] range) to its corresponding volume.
fn position_to_volume(t: f32) -> Volume {
    let curve = r!(UnevenSampleAutoCurve::new([
        (0.0, f32::NEG_INFINITY),
        (0.01, -30.0),
        (0.5, -7.0),
        (1.0, 0.0),
    ]));
    Volume::Decibels(r!(curve.sample(t.clamp(0.0, 1.0))))
}

fn apply_audio_settings(
    audio_settings: Res<AudioSettings>,
    music_audio_query: Query<Entity, With<MusicAudio>>,
    sfx_audio_query: Query<Entity, With<SfxAudio>>,
    ui_audio_query: Query<Entity, With<UiAudio>>,
    mut volume_query: Query<(Option<&mut PlaybackSettings>, Option<&mut AudioSink>)>,
) {
    // Apply music volume.
    let volume = audio_settings.music_volume();
    for entity in &music_audio_query {
        let (playback, sink) = c!(volume_query.get_mut(entity));

        if let Some(mut sink) = sink {
            sink.set_volume(volume);
        } else if let Some(mut playback) = playback {
            playback.volume = volume;
        }
    }

    // Apply SFX volume.
    let volume = audio_settings.sfx_volume();
    for entity in &sfx_audio_query {
        let (playback, sink) = c!(volume_query.get_mut(entity));

        if let Some(mut sink) = sink {
            sink.set_volume(volume);
        } else if let Some(mut playback) = playback {
            playback.volume = volume;
        }
    }

    // Apply UI volume.
    let volume = audio_settings.ui_volume();
    for entity in &ui_audio_query {
        let (playback, sink) = c!(volume_query.get_mut(entity));

        if let Some(mut sink) = sink {
            sink.set_volume(volume);
        } else if let Some(mut playback) = playback {
            playback.volume = volume;
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct MusicAudio;

impl Configure for MusicAudio {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn music_audio(audio_settings: &AudioSettings, handle: Handle<AudioSource>) -> impl Bundle {
    (
        Name::new("MusicAudio"),
        MusicAudio,
        AudioPlayer(handle),
        PlaybackSettings::LOOP.with_volume(audio_settings.music_volume()),
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct SfxAudio;

impl Configure for SfxAudio {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn sfx_audio(
    audio_settings: &AudioSettings,
    handle: Handle<AudioSource>,
    speed: f32,
) -> impl Bundle {
    (
        Name::new("SfxAudio"),
        SfxAudio,
        AudioPlayer(handle),
        PlaybackSettings::DESPAWN
            .with_volume(audio_settings.sfx_volume())
            .with_speed(speed),
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct UiAudio;

impl Configure for UiAudio {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn ui_audio(
    audio_settings: &AudioSettings,
    handle: Handle<AudioSource>,
    speed: f32,
) -> impl Bundle {
    (
        Name::new("UiAudio"),
        UiAudio,
        AudioPlayer(handle),
        PlaybackSettings::DESPAWN
            .with_volume(audio_settings.ui_volume())
            .with_speed(speed),
    )
}
