use bevy::prelude::*;

#[derive(Component)]
pub struct AntHill {
    /// Number of ants in the ant hill
    pub health: f32,

    /// Time until the next ant is spawned
    pub time_to_spawn: f32,

    /// Directional bias to spawn in
    pub spawn_bias: Vec2,
}

impl AntHill {
    pub fn new(health: f32) -> Self {
        Self {
            health,
            time_to_spawn: 0.0,
            spawn_bias: Vec2::new(
                0.0, 0.0,
            ),
        }
    }

    pub fn reduce_bias(&mut self) {
        self.spawn_bias *= 0.9;
    }

    pub fn return_ant(
        &mut self,
        ant: &mut super::ant::Ant,
    ) {
        self.health += ant.health + ant.carrying;
        ant.health = 0.0;
        ant.carrying = 0.0;
        self.spawn_bias += -ant.heading.normalize();
    }

    pub fn get_initial_spawn_heading(&self) -> Vec2 {
        let heading = Vec2::new(
            rand::random::<f32>() - 0.5,
            rand::random::<f32>() - 0.5,
        ) + self.spawn_bias;
        heading.normalize()
    }
}
