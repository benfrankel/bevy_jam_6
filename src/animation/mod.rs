pub mod backup;
pub mod lifetime;
pub mod offset;
pub mod oscillate;
pub mod shake;

use bevy::transform::systems::mark_dirty_trees;
use bevy::transform::systems::propagate_parent_transforms;
use bevy::transform::systems::sync_simple_transforms;
use bevy::ui::UiSystem;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(SaveBackupSystems, PostTransformSystems, PostColorSystems)>();

    app.add_plugins((
        backup::plugin,
        lifetime::plugin,
        offset::plugin,
        oscillate::plugin,
        shake::plugin,
    ));
}

#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
struct SaveBackupSystems;

impl Configure for SaveBackupSystems {
    fn configure(app: &mut App) {
        app.configure_sets(
            PostUpdate,
            ((UiSystem::PostLayout, PhysicsSet::Sync), Self).chain(),
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
    /// Propagate transforms before tooltip placement.
    Propagate,
    /// Apply finishing touches to GlobalTransform, like rounding to the nearest pixel.
    Finish,
}

impl Configure for PostTransformSystems {
    fn configure(app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                SaveBackupSystems,
                Self::Blend,
                Self::ApplyFacing,
                Self::Propagate,
                TooltipSystems::Placement,
                TransformSystem::TransformPropagate,
                Self::Finish,
                // GlobalTransform may be slightly out of sync with Transform at this point...
            )
                .chain(),
        );
        app.add_systems(
            PostUpdate,
            (
                mark_dirty_trees,
                propagate_parent_transforms,
                sync_simple_transforms,
            )
                .in_set(Self::Propagate),
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
        app.configure_sets(PostUpdate, (SaveBackupSystems, Self::Blend).chain());
    }
}
