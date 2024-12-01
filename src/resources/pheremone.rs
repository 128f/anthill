use bevy::prelude::*;
use kiddo::KdTree;
use kiddo::SquaredEuclidean;

#[derive(Resource)]
pub struct PheremoneData {
    pub tree: KdTree<f32, 2>,
}

impl PheremoneData {
    pub fn new() -> Self {
        Self {
            tree: KdTree::new(),
        }
    }
    pub fn insert(
        &mut self,
        id: u64,
        location: Vec2,
    ) {
        self.tree.add(
            &[location.x, location.y],
            id,
        );
    }
    pub fn remove(
        &mut self,
        position: Vec2,
        id: u64,
    ) {
        self.tree.remove(
            &[position.x, position.y],
            id,
        );
    }

    pub fn find_closest(
        &self,
        location: Vec2,
    ) -> Option<(
        f32,
        u64,
    )> {
        let mut nearest_query = self.tree.best_n_within::<SquaredEuclidean>(
            &[location.x, location.y],
            10.0,
            1,
        );
        if let Some(nearest) = nearest_query.next() {
            Some((
                nearest.distance,
                nearest.item,
            ))
        } else {
            None
        }
    }
}
