use crate::components::ant::*;
use crate::consts::*;
use bevy::prelude::*;

pub fn move_ant(
    mut query: Query<(
        &mut Transform,
        &Ant,
    )>,
    time: Res<Time>,
) {
    for (mut transform, ant) in query.iter_mut() {
        match ant.behavior {
            Behavior::Search => {
                // println!("Searching");
                // println!("Position: {:?}", transform.translation);
                transform.translation +=
                    ant.heading.extend(0.0) * ant.velocity * time.delta_seconds();
            }
            Behavior::Return => {
                transform.translation +=
                    -ant.heading.extend(0.0) * ant.velocity * time.delta_seconds();
            }
        }
    }
}

pub fn recolor_ant(
    mut query: Query<(
        &mut Sprite,
        &Ant,
    )>
) {
    for (mut sprite, ant) in query.iter_mut() {
        sprite.color = Color::srgb(
            1.0 - (ant.health / DEFAULT_HEALTH),
            0.0,
            0.0,
        );
    }
}

pub fn decay_ant(
    mut query: Query<&mut Ant>,
    time: Res<Time>,
) {
    for mut ant in query.iter_mut() {
        ant.health -= time.delta_seconds();
    }
}

pub fn remove_ant(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Ant,
    )>,
) {
    for (entity, ant) in query.iter_mut() {
        if ant.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
