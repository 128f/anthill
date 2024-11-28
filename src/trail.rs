use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// Component to mark entities that should have a trail
#[derive(Component)]
pub struct TrailEffect {
    pub effect_handle: Handle<EffectAsset>,
}

// Plugin to manage the trail effect system
pub struct TrailEffectPlugin;

impl Plugin for TrailEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_trail_effect)
            .add_systems(Update, update_trail_positions);
    }
}

// Resource to store the effect asset
#[derive(Resource)]
pub struct TrailEffectAsset(Handle<EffectAsset>);

fn setup_trail_effect(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(1.0, 0.5, 0.0, 1.0)); // Orange
    color_gradient.add_key(1.0, Vec4::new(1.0, 0.0, 0.0, 0.0)); // Fade to transparent red

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::splat(0.1));
    size_gradient.add_key(1.0, Vec2::splat(0.05));

    let writer = ExprWriter::new();
    let mut module = Module::new();

    let effect = EffectAsset::new(
        module,
        // Using graph-based API for v0.12.2
        Graph::new()
            .spawn_settings(
                writer
                    .init_position(Circle {
                        radius: writer.lit(0.1),
                        dimension: ShapeDimension::Surface,
                        ..Default::default()
                    })
                    .init_lifetime(writer.lit(0.5))
                    .init_velocity(writer.lit(Vec3::ZERO))
                    .spawn_settings(),
            )
            .render(color_gradient)
            .render_settings(RenderSettings {
                size_gradient,
                ..Default::default()
            })
            .build()
            .unwrap(),
        CompiledGraphSettings {
            max_particles: 512,
            ..Default::default()
        },
    );

    // Store the effect asset
    let effect_handle = effects.add(effect);
    commands.insert_resource(TrailEffectAsset(effect_handle));
}

fn update_trail_positions(
    mut effects: Query<(&mut Transform, &Handle<EffectAsset>)>,
    parent_query: Query<&GlobalTransform, With<TrailEffect>>,
) {
    for (mut effect_transform, _) in effects.iter_mut() {
        if let Ok(parent_transform) = parent_query.get_single() {
            effect_transform.translation = parent_transform.translation();
        }
    }
}

// Function to add trail effect to an entity
pub fn add_trail_to_entity(
    commands: &mut Commands,
    entity: Entity,
    effect_asset: &TrailEffectAsset,
) {
    commands.entity(entity).insert(TrailEffect {
        effect_handle: effect_asset.0.clone(),
    });

    // Spawn the effect entity as a child of the main entity
    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_asset.0.clone()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Name::new("Trail Effect"),
    ));
}
