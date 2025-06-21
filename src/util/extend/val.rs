use crate::prelude::*;

// TODO: Workaround for <https://github.com/bevyengine/bevy/issues/5893>.
pub trait ValExtAdd: Sized {
    fn add(
        &self,
        other: Self,
        parent_size: f32,
        target_size: Vec2,
    ) -> Result<Self, ValArithmeticError>;
}

impl ValExtAdd for Val {
    fn add(
        &self,
        other: Self,
        parent_size: f32,
        target_size: Vec2,
    ) -> Result<Self, ValArithmeticError> {
        Ok(Px(
            self.resolve(parent_size, target_size)? + other.resolve(parent_size, target_size)?
        ))
    }
}
