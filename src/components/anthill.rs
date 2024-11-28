use bevy::prelude::*;

#[derive(Component)]
pub struct AntHill {
    /// Number of ants in the ant hill
    pub count: u32,

    /// Time until the next ant is spawned
    pub time_to_spawn: f32,
}

impl AntHill {
    pub fn new(count: u32) -> Self {
        Self {
            count,
            time_to_spawn: 0.0,
        }
    }
}
