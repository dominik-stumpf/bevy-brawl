use crate::prelude::*;

use super::{cast_ability::AbilityCast, Ability};
use bevy::prelude::*;
use bevy_xpbd_3d::{
    components::{CollisionLayers, LayerMask},
    plugins::collision::{Collider, CollidingEntities},
};

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_projectile,
                despawn_projectile,
                move_projectile,
                control_projectile_timer,
                handle_projectile_collision,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component)]
struct Fireball {
    life_timer: Timer,
    is_alive: bool,
}

impl Default for Fireball {
    fn default() -> Self {
        Self {
            life_timer: Timer::from_seconds(1.4, TimerMode::Once),
            is_alive: true,
        }
    }
}

fn spawn_projectile(
    mut events: EventReader<AbilityCast>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.read() {
        if event.ability == Ability::Fireball {
            let direction = (event.cast_origin.xz() - event.cast_destination.xz()).normalize();

            commands.spawn((
                Name::new("Fireball"),
                Fireball::default(),
                Collider::sphere(0.6),
                CollisionLayers::new(GameLayer::Projectile, LayerMask::ALL),
                PbrBundle {
                    mesh: meshes.add(Sphere::new(1.0)),
                    material: materials.add(StandardMaterial {
                        emissive: Color::ORANGE_RED * 256.0,
                        ..default()
                    }),
                    transform: Transform::from_translation(event.cast_origin)
                        .with_scale(Vec3::new(0.3, 0.3, 0.3))
                        .with_rotation(Quat::from_euler(
                            EulerRot::XYZ,
                            0.0,
                            direction.angle_between(Vec2::X),
                            0.0,
                        )),
                    ..default()
                },
            ));
        }
    }
}

fn move_projectile(time: Res<Time>, mut projectile_query: Query<&mut Transform, With<Fireball>>) {
    for mut projectile_transform in &mut projectile_query {
        let direction = -projectile_transform.local_x();
        projectile_transform.translation += direction * 20.0 * time.delta_seconds();
    }
}

fn despawn_projectile(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Fireball)>,
    mut play_sfx: EventWriter<audio::EventPlaySFX>,
) {
    for (entity, projectile) in &projectile_query {
        if !projectile.is_alive {
            println!("audio");
            play_sfx.send(audio::EventPlaySFX::new(
                audio::SFXKind::MagicMissileExplosion,
            ));
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn control_projectile_timer(time: Res<Time>, mut projectile_query: Query<&mut Fireball>) {
    for mut projectile in &mut projectile_query {
        projectile.life_timer.tick(time.delta());
        if projectile.life_timer.just_finished() {
            projectile.is_alive = false;
        }
    }
}

fn handle_projectile_collision(
    mut projectile_query: Query<(&CollidingEntities, &mut Fireball)>,
    terrain_query: Query<&Terrain>,
) {
    for (colliding_entities, mut projectile) in projectile_query.iter_mut() {
        for collided_entity in colliding_entities.iter() {
            if terrain_query.contains(*collided_entity) {
                projectile.is_alive = false;
                break;
            }
        }
    }
}
