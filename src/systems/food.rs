use crate::components;
use crate::consts::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::ActiveCollisionTypes;
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::Sensor;

pub fn spawn_food(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut food_spawner: Query<&mut components::food::Spawner>,
    time: Res<Time>,
) {
    let mut spawner = food_spawner.single_mut();
    if !spawner.timer.tick(time.delta()).just_finished() {
        return;
    }
    let food = components::food::Food::random();
    let path = format!(
        "{}.png",
        food.get_sprite()
    );
    // TODO: preload this somewhere else
    let texture = asset_server.load(path);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(
            50, 30,
        ),
        4,
        1,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let remaining_health = food.health;

    commands
        .spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_xyz(
                    rand::random::<f32>() * FOOD_SPAWN_WIDTH - FOOD_SPAWN_WIDTH / 2.0,
                    rand::random::<f32>() * FOOD_SPAWN_HEIGHT - FOOD_SPAWN_HEIGHT / 2.0,
                    2.0,
                )
                .with_scale(Vec3::splat(1.0)),
                ..Default::default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
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
    // .insert(Food::random());
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
