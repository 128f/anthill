use bevy::ecs::event;
use bevy::ui::UiImage;
use bevy::window::PrimaryWindow;
use bevy::{ecs::event::EventIterator, prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::prelude::*;
// use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;
use components::{
    ant::{self, Ant},
    anthill::AntHill,
    food::Food,
};
use rand;
use resources::dropper::{DropRequest, FoodType, SelectedFood};
use resources::textures::FoodTextures;

pub mod components;
pub mod consts;
pub mod resources;
pub mod systems;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Running,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Running)
                .load_collection::<FoodTextures>(),
        )
        .add_plugins(HanabiPlugin)
        // .add_plugins(EguiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(resources::pheremone::PheremoneData::new())
        .insert_resource(
            SelectedFood {
                selected: FoodType::Bird,
            },
        )
        .add_event::<DropRequest>()
        .add_systems(
            Startup,
            (
                setup_camera,
                systems::anthill::build_trail_effect,
                setup_grid,
                systems::anthill::setup_anthill,
                setup_food_spawner,
                ui_setup,
            ),
        )
        .add_systems(
            Update,
            (
                // ui_example_system,
                toolbar_system,
                mouse_button_input,
                systems::food::spawn_food,
                systems::food::update_food_texture,
                systems::food::update_food_text,
                systems::food::remove_depleted_food,
                systems::anthill::spawn_ant,
                systems::anthill::update_anthill_text,
                systems::ant::increment_lifetime,
                systems::ant::tilt_ant,
                systems::ant::apply_pheremones.after(systems::ant::tilt_ant),
                systems::ant::adjust_debug_image.after(systems::ant::apply_pheremones),
                systems::ant::move_ant.after(systems::ant::apply_pheremones),
                systems::ant::drop_pheremones,
                systems::ant::decay_ant,
                systems::ant::remove_ant,
                systems::ant::recolor_ant,
                systems::pheremone::decay_pheremones.after(systems::ant::apply_pheremones),
                systems::pheremone::remove_decayed_pheremones
                    .after(systems::pheremone::decay_pheremones),
                detect_collisions,
            )
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            Update,
            (systems::anthill::reduce_anthill_bias).run_if(in_state(GameState::Running)),
        )
        .run();
}

fn setup_food_spawner(mut commands: Commands) {
    commands.spawn((components::food::Spawner::new(),));
}

#[derive(Component)]
struct ToolBarButton {
    food_type: FoodType,
}

pub fn ui_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // Load the image asset
    let bird_icon = asset_server.load("../assets/icons/bird_icon.png");
    let hotdog_icon = asset_server.load("../assets/icons/hotdog_icon.png");
    let marshmallow_icon = asset_server.load("../assets/icons/marshmallow_icon.png");

    commands
        .spawn(
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(30.0), // Full-width for the parent node
                    height: Val::Percent(10.0), // Full-height for the parent node
                    ..Default::default()
                },

                z_index: ZIndex::Global(59),
                ..Default::default()
            },
        )
        .with_children(
            |parent| {
                parent
                    .spawn(
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(100.0),  // Full-width for the parent node
                                height: Val::Percent(100.0), // Full-height for the parent node
                                ..Default::default()
                            },
                            image: UiImage::from(bird_icon),
                            ..Default::default()
                        },
                    )
                    .insert(
                        ToolBarButton {
                            food_type: FoodType::Bird,
                        },
                    );
                parent
                    .spawn(
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(100.0),  // Full-width for the parent node
                                height: Val::Percent(100.0), // Full-height for the parent node
                                ..Default::default()
                            },
                            image: UiImage::from(hotdog_icon),
                            ..Default::default()
                        },
                    )
                    .insert(
                        ToolBarButton {
                            food_type: FoodType::Hotdog,
                        },
                    );
                parent
                    .spawn(
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(100.0),  // Full-width for the parent node
                                height: Val::Percent(100.0), // Full-height for the parent node
                                ..Default::default()
                            },
                            image: UiImage::from(marshmallow_icon),
                            ..Default::default()
                        },
                    )
                    .insert(
                        ToolBarButton {
                            food_type: FoodType::Marshmallow,
                        },
                    );
            },
        );
}

fn toolbar_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &ToolBarButton,
        ),
        (
            Changed<Interaction>,
            With<Button>,
        ),
    >,
    mut current_selected: ResMut<SelectedFood>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => current_selected.selected = button.food_type.clone(),
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>,
    camera_query: Query<
        (
            &Camera,
            &GlobalTransform,
        ),
        With<Camera>,
    >,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut event_writer: EventWriter<DropRequest>,
    current_selected: Res<SelectedFood>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            let (camera, camera_transform) = camera_query.single();
            println!(
                "Cursor is inside the primary window, at {:?}",
                position
            );
            let world_pos = camera.viewport_to_world_2d(
                camera_transform,
                position,
            );
            if world_pos.is_none() {
                // log::warn!("Cursor is outside the camera's view");
                return;
            }
            let world_pos = world_pos.unwrap();
            event_writer.send(
                DropRequest {
                    position: world_pos,
                    food_type: current_selected.selected.clone(),
                },
            );
        }
        println!("Left mouse button pressed");
    }
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
