use bevy::{ecs::event::EventIterator, prelude::*, render::camera::ScalingMode};
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;
use components::{
    ant::{self, Ant},
    anthill::AntHill,
    food::Food,
};
use rand;

pub mod components;
pub mod consts;
pub mod resources;
pub mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(HanabiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(resources::pheremone::PheremoneData::new())
        .add_systems(
            Startup,
            (
                setup_camera,
                systems::anthill::build_trail_effect,
                setup_grid,
                systems::anthill::setup_anthill,
                setup_food_spawner,
            ),
        )
        .add_systems(
            Update,
            (
                systems::food::spawn_food,
                systems::food::update_food_text,
                systems::anthill::spawn_ant,
                systems::ant::increment_lifetime,
                systems::ant::tilt_ant,
                systems::ant::apply_pheremones.after(systems::ant::tilt_ant),
                systems::ant::adjust_debug_image.after(systems::ant::apply_pheremones),
                systems::ant::move_ant.after(systems::ant::apply_pheremones),
                systems::ant::drop_pheremones,
                systems::ant::decay_ant,
                systems::ant::remove_ant,
                systems::ant::recolor_ant,
                systems::pheremone::decay_pheremones,
                systems::pheremone::remove_decayed_pheremones
                    .after(systems::pheremone::decay_pheremones),
                detect_collisions,
            ),
        )
        .run();
}

fn setup_food_spawner(mut commands: Commands) {
    commands.spawn((components::food::Spawner::new(),));
}

fn setup_camera(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn a 2D camera
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 1.0;
    camera.projection.scaling_mode = ScalingMode::FixedVertical(300.);
    commands.spawn(camera);
}

fn setup_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("ground.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(25),
        4,
        4,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for i in -50..50 {
        for k in -50..50 {
            let idx = rand::random::<usize>() % 4;
            commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_translation(
                        Vec2::new(
                            (i as f32) * 25.0,
                            (k as f32) * 25.0,
                        )
                        .extend(0.0),
                    ),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: idx,
                },
            ));
        }
    }
}

pub struct CollisionMap<'a> {
    pub colliding_entities: Vec<&'a Entity>,
    pub colliding_pairs: std::collections::HashMap<&'a Entity, &'a Entity>,
}

impl<'a> CollisionMap<'a> {
    // We build a list of all colliding entities and a map that relates them to each other
    // We are paying O(n) up-front for this convenience
    pub fn new(collision_events: EventIterator<'a, CollisionEvent>) -> Self {
        let mut colliding_entities = vec![];
        let mut colliding_map = std::collections::HashMap::new();
        for collision_event in collision_events {
            if let CollisionEvent::Started(handle1, handle2, _) = collision_event {
                colliding_entities.push(handle1);
                colliding_entities.push(handle2);
                colliding_map.insert(
                    handle1, handle2,
                );
                colliding_map.insert(
                    handle2, handle1,
                );
            }
            // println!("Received collision event: {:?}", collision_event);
        }
        CollisionMap {
            colliding_entities,
            colliding_pairs: colliding_map,
        }
    }
}

pub fn detect_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    // TODO: combine these with Option wrappers for Health and DealsDamage
    mut ants: Query<(
        Entity,
        &mut Ant,
    )>,
    mut food_objects: Query<(
        Entity,
        &Transform,
        &mut Food,
    )>,
    mut anthills: Query<(
        Entity,
        &mut AntHill,
    )>,
) {
    let events = collision_events.read();
    let mut collision_map = CollisionMap::new(events);

    for e in collision_map.colliding_entities {
        let partner = collision_map
            .colliding_pairs
            .get_mut(e)
            .expect("Colliding map improperly created");
        let ant = ants.get_mut(*e);
        let food = food_objects.get_mut(**partner);
        let anthill = anthills.get_mut(**partner);
        // println!(
        //     "Ant: {:?}, Food: {:?}",
        //     ant.is_ok(),
        //     food.is_ok()
        // );
        if ant.is_ok() && anthill.is_ok() {
            // println!("Collision detected between ant and anthill");
            let (_, mut ant) = ant.unwrap();
            let (_, mut anthill) = anthill.unwrap();
            if ant.is_returning() {
                anthill.return_ant(&mut ant);
            }
        } else if ant.is_ok() && food.is_ok() {
            // println!("Collision detected between ant and food");
            let (_, mut ant) = ant.unwrap();
            let (_, food_location, mut food) = food.unwrap();
            if ant.is_returning() {
                continue;
            }
            let amount = ant.capacity();
            let actual = food.consume(amount);
            ant.fill(actual);
            ant.set_returning();
            ant.food_location = Some(food_location.translation.truncate());
        }
    }
}
