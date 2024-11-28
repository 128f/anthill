use bevy::prelude::*;

use crate::consts::*;

pub enum Behavior {
    Search,
    Return,
}

#[derive(Component)]
pub struct Ant {
    /// Normed vector representing the heading of the ant
    pub heading: Vec2,
    /// Speed of the ant
    pub velocity: f32,
    /// Health of the ant
    pub health: f32,
    /// Current behavior of the ant
    pub behavior: Behavior,
}

impl Ant {
    pub fn new(heading: Vec2, velocity: f32) -> Self {
        Self {
            heading,
            velocity,
            health: DEFAULT_HEALTH,
            behavior: Behavior::Search,
        }
    }
    pub fn random() -> Self {
        let heading =
            Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize();
        let velocity = DEFAULT_VELOCITY;
        Self {
            heading,
            velocity,
            health: DEFAULT_HEALTH,
            behavior: Behavior::Search,
        }
    }
}
