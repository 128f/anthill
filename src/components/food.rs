use crate::consts::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Spawner {
    pub timer: Timer,
}

impl Spawner {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(
                FOOD_SPAWN_RATE,
                TimerMode::Repeating,
            ),
        }
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
}
