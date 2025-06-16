use crate::core::physics::GameLayer;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Faction>();
}

#[derive(Component, Reflect, Debug, Copy, Clone, Eq, PartialEq)]
#[reflect(Component)]
pub enum Faction {
    Player,
    Enemy,
}

impl Configure for Faction {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

impl Faction {
    pub fn layer(&self) -> GameLayer {
        match self {
            Self::Player => GameLayer::Player,
            Self::Enemy => GameLayer::Enemy,
        }
    }

    pub fn opponent(&self) -> Self {
        match self {
            Self::Player => Self::Enemy,
            Self::Enemy => Self::Player,
        }
    }
}
