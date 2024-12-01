use bevy::prelude::*;

use crate::consts::DEFAULT_PHEREMONE_STRENGTH;

#[derive(Component)]
pub struct Pheremone {
    pub position: Vec2,
    pub heading: Vec2,
    pub strength: f32,
}

impl Pheremone {
    pub fn new(
        position: Vec2,
        heading: Vec2,
    ) -> Self {
        Self {
            position,
            heading,
            strength: DEFAULT_PHEREMONE_STRENGTH,
        }
    }
}
