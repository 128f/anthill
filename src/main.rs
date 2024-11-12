use bevy::{log::tracing_subscriber::fmt::time, prelude::*};
use rand;

const DEFAULT_VELOCITY: f32 = 20.0;
const DEFAULT_HEALTH: f32 = 5.0;

enum Behavior {
    Search,
    Return,
}

#[derive(Component)]
struct Ant {
    /// Normed vector representing the heading of the ant
    heading: Vec2,
    /// Speed of the ant
    velocity: f32,
    /// Health of the ant
    health: f32,
    /// Current behavior of the ant
    behavior: Behavior,
}

#[derive(Component)]
struct AntHill {
    /// Number of ants in the ant hill
    count: u32,

    /// Time until the next ant is spawned
    time_to_spawn: f32,
}

impl AntHill {
    fn new(count: u32) -> Self {
        Self {
            count,
            time_to_spawn: 0.0,
        }
    }
}

impl Ant {
    fn new(heading: Vec2, velocity: f32) -> Self {
        Self {
            heading,
            velocity,
            health: DEFAULT_HEALTH,
            behavior: Behavior::Search,
        }
    }
    fn random() -> Self {
        let heading =
            Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize();
        let velocity = DEFAULT_VELOCITY;
        Self {
            heading,
            velocity,
            health: DEFAULT_HEALTH,
            behavior: Behavior::Search,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_ant, spawn_ant, decay_ant, remove_ant, recolor_ant),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
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
        AntHill::new(10),
    ));
}

fn spawn_ant(mut commands: Commands, mut query: Query<&mut AntHill>, time: Res<Time>) {
    let mut ant_hill = query.single_mut();
    if ant_hill.count > 0 && ant_hill.time_to_spawn <= 0.0 {
        ant_hill.count -= 1;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..default()
            },
            Ant::random(),
        ));
    } else {
        ant_hill.time_to_spawn -= time.delta_seconds();
    }
}

fn move_ant(mut query: Query<(&mut Transform, &Ant)>, time: Res<Time>) {
    for (mut transform, ant) in query.iter_mut() {
        match ant.behavior {
            Behavior::Search => {
                transform.translation +=
                    ant.heading.extend(0.0) * ant.velocity * time.delta_seconds();
            }
            Behavior::Return => {
                transform.translation +=
                    -ant.heading.extend(0.0) * ant.velocity * time.delta_seconds();
            }
        }
    }
}

fn recolor_ant(mut query: Query<(&mut Sprite, &Ant)>) {
    for (mut sprite, ant) in query.iter_mut() {
        sprite.color = Color::srgb(1.0 - (ant.health / DEFAULT_HEALTH), 0.0, 0.0);
    }
}

fn decay_ant(mut query: Query<&mut Ant>, time: Res<Time>) {
    for mut ant in query.iter_mut() {
        ant.health -= time.delta_seconds();
    }
}

fn remove_ant(mut commands: Commands, mut query: Query<(Entity, &Ant)>, time: Res<Time>) {
    for (entity, ant) in query.iter_mut() {
        if ant.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
