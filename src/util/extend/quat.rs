use crate::prelude::*;

// TODO: Workaround for <https://github.com/bevyengine/bevy/issues/14525>.
pub trait QuatConversionExt {
    fn to_quat(self) -> Quat;
    fn to_dir2(self) -> Dir2;
    fn to_rot2(self) -> Rot2;
}

impl QuatConversionExt for Quat {
    fn to_quat(self) -> Quat {
        self
    }

    fn to_dir2(self) -> Dir2 {
        self.to_rot2().to_dir2()
    }

    fn to_rot2(self) -> Rot2 {
        Rot2::radians(self.to_scaled_axis().z)
    }
}

impl QuatConversionExt for Rot2 {
    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(Rot2::IDENTITY.angle_to(self))
    }

    fn to_dir2(self) -> Dir2 {
        self.normalize();
        Dir2::from_xy_unchecked(self.cos, self.sin)
    }

    fn to_rot2(self) -> Rot2 {
        self
    }
}

impl QuatConversionExt for Dir2 {
    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(self.to_angle())
    }

    fn to_dir2(self) -> Dir2 {
        self
    }

    fn to_rot2(self) -> Rot2 {
        self.rotation_from_x()
    }
}
