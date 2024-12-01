use bevy::prelude::*;

#[derive(Component)]
pub struct AntHill {
    /// Number of ants in the ant hill
    pub health: f32,

    /// Time until the next ant is spawned
    pub time_to_spawn: f32,
}

impl AntHill {
    pub fn new(health: f32) -> Self {
        Self {
            health,
            time_to_spawn: 0.0,
        }
    }
    pub fn return_ant(
        &mut self,
        ant: &mut super::ant::Ant,
    ) {
        self.health += ant.health + ant.carrying;
        ant.health = 0.0;
        ant.carrying = 0.0;
    }
}
