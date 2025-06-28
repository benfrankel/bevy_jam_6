pub mod backup;
pub mod fade;
pub mod lifetime;
pub mod offset;
pub mod oscillate;
pub mod shake;

use bevy::ui::UiSystem;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(BackupSystems, PostTransformSystems, PostColorSystems)>();

    app.add_plugins((
        backup::plugin,
        fade::plugin,
        lifetime::plugin,
        offset::plugin,
        oscillate::plugin,
        shake::plugin,
    ));
}

#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
enum BackupSystems {
    Insert,
    Save,
}

impl Configure for BackupSystems {
    fn configure(app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                (UiSystem::PostLayout, PhysicsSet::Sync, Self::Insert),
                Self::Save,
            )
                .chain(),
        );
    }
}

/// [`Transform`] post-processing steps for the [`PostUpdate`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostTransformSystems {
    /// Blend via transform multiplication (add translation, add rotation, multiply scale).
    Blend,
    /// Apply facing (may multiply translation.x by -1).
    ApplyFacing,
    /// Apply finishing touches to [`GlobalTransform`], like rounding to the nearest pixel.
    Finish,
}

impl Configure for PostTransformSystems {
    fn configure(app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                BackupSystems::Save,
                Self::Blend,
                Self::ApplyFacing,
                TransformSystem::TransformPropagate,
                Self::Finish,
                // GlobalTransform may be slightly out of sync with Transform at this point...
            )
                .chain(),
        );
    }
}

/// [`Color`] post-processing steps for the [`PostUpdate`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostColorSystems {
    /// Blend via color multiplication (multiply RGBA).
    Blend,
}

impl Configure for PostColorSystems {
    fn configure(app: &mut App) {
        app.configure_sets(PostUpdate, (BackupSystems::Save, Self::Blend).chain());
    }
}
