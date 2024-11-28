use crate::components;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub fn spawn_ant(
    mut commands: Commands,
    mut query: Query<&mut components::anthill::AntHill>,
    time: Res<Time>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Create a color gradient for the particles
    let mut gradient = Gradient::new();
    gradient.add_key(
        0.0,
        Vec4::new(
            1.0, 0., 0., 1.0,
        ),
    );
    gradient.add_key(
        1.0,
        Vec4::new(
            0.5, 0.5, 1.0, 0.0,
        ),
    );

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(
        Attribute::AGE,
        age,
    );

    let lifetime = writer.lit(0.2).expr();
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        lifetime,
    );

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(0.05).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(0.1).expr(),
    };

    let mut module = writer.finish();

    let round = RoundModifier::constant(
        &mut module,
        2.0 / 3.0,
    );

    let spawner = Spawner::rate(30.0.into());
    let effect = effects.add(
        EffectAsset::new(
            4096, spawner, module,
        )
        .with_name("2d")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .render(
            SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec3::splat(1.0)),
                screen_space_size: false,
            },
        )
        .render(ColorOverLifetimeModifier { gradient })
        .render(round),
    );

    let mut ant_hill = query.single_mut();
    if ant_hill.count > 0 && ant_hill.time_to_spawn <= 0.0 {
        ant_hill.count -= 1;
        println!("Spawning ant");

        // Spawn an instance of the particle effect, and override its Z layer to
        // be above the reference white square previously spawned.
        commands
            .spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect).with_z_layer_2d(Some(0.1)),
                    transform: Transform::from_translation(
                        Vec3::new(
                            0.0, 0.0, 0.0,
                        ),
                    ),
                    ..Default::default()
                },
                // SpriteBundle {
                //     sprite: Sprite {
                //         color: Color::srgb(0.0, 1.0, 0.0),
                //         ..Default::default()
                //     },
                //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                //         .with_scale(Vec3::splat(1.0)),
                //     ..Default::default()
                // },
                components::ant::Ant::random(),
            ))
            .insert(Name::new("effect:2d"));
    } else {
        ant_hill.time_to_spawn -= time.delta_seconds();
    }
}
