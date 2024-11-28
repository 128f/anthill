use bevy::render::texture::ImageLoaderSettings;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_hanabi::prelude::*;
use rand;

pub mod components;
pub mod consts;
pub mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(HanabiPlugin)
        .add_systems(
            Startup,
            (setup_camera, setup_grid, setup_anthill, setup_food_spawner),
        )
        .add_systems(
            Update,
            (
                systems::food::spawn_food,
                systems::anthill::spawn_ant,
                systems::ant::move_ant,
                systems::ant::decay_ant,
                systems::ant::remove_ant,
                systems::ant::recolor_ant,
            ),
        )
        .run();
}

fn setup_food_spawner(mut commands: Commands) {
    commands.spawn((components::food::Spawner::new(),));
}

fn setup_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(25), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for i in -50..50 {
        for k in -50..50 {
            let idx = rand::random::<usize>() % 4;
            commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_translation(
                        Vec2::new((i as f32) * 25.0, (k as f32) * 25.0).extend(0.0),
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

// AntHill

fn setup_anthill(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(5.0, 5.0, 1.0), // Size of the square
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..default()
        },
        components::anthill::AntHill::new(10000),
    ));
}
