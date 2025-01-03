use std::string;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct FoodTextures {
    #[asset(
        texture_atlas_layout(
            tile_size_x = 50,
            tile_size_y = 30,
            columns = 4,
            rows = 1
        )
    )]
    pub food_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "bird.png")]
    pub bird: Handle<Image>,
    #[asset(path = "hotdog.png")]
    pub hotdog: Handle<Image>,
    #[asset(path = "marshmallow.png")]
    pub marshmallow: Handle<Image>,
}

impl FoodTextures {
    pub fn get_texture(
        &self,
        name: &str,
    ) -> Handle<Image> {
        match name {
            name if name == "bird" => self.bird.clone(),
            name if name == "hotdog" => self.hotdog.clone(),
            name if name == "marshmallow" => self.marshmallow.clone(),
            _ => panic!("Invalid food name"),
        }
    }
}
