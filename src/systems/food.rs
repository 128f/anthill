use crate::components;
use crate::components::food::Food;
use crate::consts::*;
use crate::resources;
use bevy::prelude::*;
use bevy_rapier2d::prelude::ActiveCollisionTypes;
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::Sensor;

pub fn spawn_food(
    mut commands: Commands,
    // mut food_spawner: Query<&mut components::food::Spawner>,
    food_textures: Res<resources::textures::FoodTextures>,
    mut event_reader: EventReader<resources::dropper::DropRequest>,
    // time: Res<Time>,
) {
    // let mut spawner = food_spawner.single_mut();
    // if !spawner.timer.tick(time.delta()).just_finished() {
    //     return;
    // }
    let event = event_reader.read().next();
    if event.is_none() {
        return;
    }
    let event = event.unwrap();
    let food = components::food::Food::from_enum(event.food_type.clone());

    let remaining_health = food.health;
    commands
        .spawn((
            SpriteBundle {
                texture: food_textures.get_texture(food.get_sprite()).clone(),
                transform: Transform::from_xyz(
                    event.position.x,
                    event.position.y,
                    2.0,
                )
                .with_scale(Vec3::splat(1.0)),
                ..Default::default()
            },
            TextureAtlas {
                layout: food_textures.food_layout.clone(),
                index: 0,
            },
            food,
        ))
        .insert(
            Collider::cuboid(
                15.0, 15.0,
            ),
        )
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::all())
        .with_children(
            |parent| {
                parent.spawn(
                    Text2dBundle {
                        transform: Transform::from_xyz(
                            0.0, 0.0, 3.0,
                        ),
                        text: Text::from_section(
                            remaining_health.to_string(),
                            TextStyle {
                                font_size: 20.0,
                                color: Color::srgba(
                                    0.0, 1.0, 0.0, 1.0,
                                ),
                                ..Default::default()
                            },
                        ),
                        ..Default::default()
                    },
                );
            },
        );
}

pub fn remove_depleted_food(
    mut commands: Commands,
    query: Query<(
        Entity,
        &Food,
    )>,
) {
    for (entity, food) in query.iter() {
        if food.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn update_food_texture(
    mut query: Query<(
        &mut TextureAtlas,
        &Food,
    )>
) {
    for (mut texture_atlas, food) in query.iter_mut() {
        let consumed = food.percentage_consumed();
        let index = (consumed * 4.0).floor() as usize;
        texture_atlas.index = index;
    }
}

pub fn update_food_text(
    mut query: Query<(
        &Children,
        &components::food::Food,
    )>,
    mut text_query: Query<&mut Text>,
) {
    for (&ref children, &ref food) in query.iter_mut() {
        for &child in children.iter() {
            let mut text = text_query.get_mut(child).unwrap();
            let formatted_health = format!(
                "{:.0}",
                food.health
            );
            text.sections[0].value = formatted_health;
        }
    }
}
