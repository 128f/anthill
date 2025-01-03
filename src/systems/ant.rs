use std::f32::consts::PI;

use crate::components::ant::*;
use crate::components::pheremone::Pheremone;
use crate::consts::*;
use crate::resources::pheremone::PheremoneData;
use bevy::{prelude::*, sprite};

pub fn move_ant(
    mut query: Query<(
        &mut Transform,
        &Ant,
    )>,
    time: Res<Time>,
) {
    for (mut transform, ant) in query.iter_mut() {
        transform.translation +=
            ant.heading.extend(0.0).normalize() * ant.velocity * time.delta_seconds();
    }
}

pub fn apply_pheremones(
    mut query: Query<(
        &Transform,
        &mut Ant,
    )>,
    pheremone_query: Query<&Pheremone>,
    pheremone_data: Res<PheremoneData>,
) {
    for (transform, mut ant) in query.iter_mut() {
        let closest = pheremone_data.find_closest(transform.translation.truncate());
        if ant.is_returning() {
            continue;
        }
        if closest.is_none() {
            ant.set_searching();
            continue;
        }
        let (_, entity_id) = closest.unwrap();
        // println!(
        //     "looking up {}",
        //     entity_id
        // );
        if let Ok(pheremone) = pheremone_query.get(Entity::from_bits(entity_id)) {
            // println!("Found pheremone");
            // println!(
            //     "Pheremone Heading: {:?}",
            //     pheremone.heading
            // );
            ant.set_following();
            ant.heading += pheremone.heading.normalize();
        } else {
            // println!(
            //     "Failed to lookup pheremone {}",
            //     Entity::from_bits(entity_id)
            // );
        }
    }
}

pub fn tilt_ant(
    mut query: Query<(
        &mut Transform,
        &mut Ant,
    )>
) {
    for (transform, mut ant) in query.iter_mut() {
        if ant.lifetime % 0.2 > 0.05 {
            continue;
        }
        match ant.behavior {
            Behavior::Search => {
                let random_angle = PI / 4.0 - PI / 2.0 * rand::random::<f32>();
                ant.heading = Quat::from_rotation_z(random_angle)
                    .mul_vec3(ant.heading.extend(0.))
                    .truncate()
                    .normalize();
            }
            Behavior::Follow => {}
            Behavior::Return => {
                let heading = ant.home_direction(&transform.translation).normalize();
                ant.heading = heading.truncate();
            }
        }
    }
}

pub fn adjust_debug_image(
    mut query: Query<(
        &mut Transform,
        &Ant,
        &Children,
    )>,
    mut debug_query: Query<
        (
            &mut Transform,
            &Sprite,
        ),
        Without<Ant>,
    >,
) {
    for (_, ant, children) in query.iter_mut() {
        for child in children.iter() {
            let mut debug_transform = debug_query.get_mut(*child).unwrap().0;
            let angle = ant.heading.y.atan2(ant.heading.x) - PI / 2.0;
            debug_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

pub fn increment_lifetime(
    mut query: Query<&mut Ant>,
    time: Res<Time>,
) {
    for mut ant in query.iter_mut() {
        ant.lifetime += time.delta_seconds();
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
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn drop_pheremones(
    mut commands: Commands,
    query: Query<(
        &Transform,
        &Ant,
    )>,
    mut pheremone_data: ResMut<PheremoneData>,
    // asset_server: ResMut<AssetServer>,
) {
    for (transform, ant) in query.iter() {
        if ant.is_returning() && ant.food_location.is_some() {
            let direction = ant.home_direction(&transform.translation).normalize();
            let pheremone = Pheremone::new(
                transform.translation.truncate(),
                -direction.truncate(),
            );
            // let texture = asset_server.load("pheremonedebug.png");
            let mut sprite_location = transform.translation.truncate().extend(0.0);
            sprite_location.z += 100.0;
            let spawned_entity = commands.spawn((
                // SpriteBundle {
                //     texture,
                //     transform: Transform::from_translation(sprite_location),
                //     ..Default::default()
                // },
                pheremone,
            ));
            pheremone_data.insert(
                spawned_entity.id().to_bits(),
                transform.translation.truncate(),
            );
        }
    }
}
