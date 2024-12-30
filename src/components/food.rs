use crate::consts::*;
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
    pub index: usize,
}

impl Food {
    pub fn random() -> Self {
        Self {
            health: DEFAULT_FOOD_HEALTH,
            index: rand::random::<usize>() % 3,
        }
    }

    pub fn get_sprite(&self) -> &str {
        match self.index {
            0 => "bird",
            1 => "hotdog",
            2 => "marshmallow",
            _ => panic!("Invalid food index"),
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
