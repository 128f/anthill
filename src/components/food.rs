use crate::{consts::*, resources::dropper::FoodType};
use bevy::prelude::*;

#[derive(Component)]
pub struct Spawner {
    pub timer: Timer,
}

impl Spawner {
    pub fn new() -> Self {
        let mut timer = Timer::from_seconds(
            FOOD_SPAWN_RATE,
            TimerMode::Repeating,
        );
        timer.set_elapsed(timer.duration());
        Self { timer }
    }
}

#[derive(Component)]
pub struct Food {
    pub health: f32,
    pub food_type: FoodType,
}

impl Food {
    // pub fn random() -> Self {
    //     Self {
    //         health: DEFAULT_FOOD_HEALTH,
    //         food_type: rand::random::<usize>() % 3,
    //     }
    // }

    pub fn from_enum(food_type: FoodType) -> Self {
        Self {
            health: DEFAULT_FOOD_HEALTH,
            food_type,
        }
    }

    pub fn get_sprite(&self) -> &str {
        match self.food_type {
            FoodType::Bird => "bird",
            FoodType::Hotdog => "hotdog",
            FoodType::Marshmallow => "marshmallow",
        }
    }

    pub fn percentage_consumed(&self) -> f32 {
        1.0 - self.health / DEFAULT_FOOD_HEALTH
    }

    pub fn consume(
        &mut self,
        amount: f32,
    ) -> f32 {
        let consumed = amount.min(self.health);
        self.health -= amount;
        self.health = self.health.max(0.0);
        consumed
    }
}
