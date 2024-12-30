use crate::components::pheremone::Pheremone;
use crate::resources::pheremone::PheremoneData;
use bevy::prelude::*;

pub fn remove_decayed_pheremones(
    query: Query<(
        Entity,
        &Pheremone,
    )>,
    mut pheremone_data: ResMut<PheremoneData>,
    mut commands: Commands,
) {
    for (entity, pheremone) in query.iter() {
        if pheremone.strength <= 0.0 {
            let id = entity.to_bits();
            commands.entity(entity).despawn_recursive();
            pheremone_data.remove(
                pheremone.position,
                id,
            );
        }
    }
}

pub fn decay_pheremones(
    mut query: Query<&mut Pheremone>,
    time: Res<Time>,
) {
    for mut pheremone in query.iter_mut() {
        pheremone.strength -= time.delta_seconds();
    }
}
