use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FoodType {
    Bird,
    Hotdog,
    Marshmallow,
}

#[derive(Resource)]
pub struct SelectedFood {
    pub selected: FoodType,
}

#[derive(Event)]
pub struct DropRequest {
    pub position: Vec2,
    pub food_type: FoodType,
}
