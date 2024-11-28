use crate::components;
use crate::consts::*;
use bevy::prelude::*;

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
    let path = format!("{}.png", food.get_sprite());
    // TODO: preload this somewhere else
    let texture = asset_server.load(path);
    let layout = TextureAtlasLayout::from_grid(UVec2::new(50, 30), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
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
    ));
    // .insert(Food::random());
}
