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
    /// lifetime of the ant
    pub lifetime: f32,
    /// location of home
    pub home_location: Vec2,
    /// location of found food
    pub food_location: Option<Vec2>,
    /// how much we are carrying
    pub carrying: f32,
}

impl Ant {
    pub fn new(
        heading: Vec2,
        velocity: f32,
        home_location: Vec2,
    ) -> Self {
        Self {
            heading,
            velocity,
            health: DEFAULT_HEALTH,
            behavior: Behavior::Search,
            lifetime: 0.0,
            home_location,
            food_location: None,
            carrying: 0.0,
        }
    }
    pub fn random(home_location: Vec2) -> Self {
        let heading = Vec2::new(
            rand::random::<f32>() - 0.5,
            rand::random::<f32>() - 0.5,
        )
        .normalize();
        let velocity = DEFAULT_VELOCITY;
        Self {
            heading,
            velocity,
            health: DEFAULT_HEALTH,
            behavior: Behavior::Search,
            lifetime: 0.0,
            home_location,
            food_location: None,
            carrying: 0.0,
        }
    }
    pub fn capacity(&self) -> f32 {
        (DEFAULT_HEALTH - self.health) + DEFAULT_CARRYING_CAPACITY
    }
    pub fn fill(
        &mut self,
        amount: f32,
    ) {
        if self.health + amount > DEFAULT_HEALTH {
            self.carrying = self.health + amount - DEFAULT_HEALTH;
            self.health = DEFAULT_HEALTH;
        } else {
            self.health += amount;
            self.carrying = 0.0;
        }
    }
    pub fn home_direction(
        &self,
        from: &Vec3,
    ) -> Vec3 {
        (self.home_location - from.truncate())
            .normalize()
            .extend(0.0)
    }

    pub fn set_returning(&mut self) {
        self.behavior = Behavior::Return;
    }
    pub fn is_returning(&self) -> bool {
        match self.behavior {
            Behavior::Return => true,
            _ => false,
        }
    }
}
