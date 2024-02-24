use crate::{camera::utils::lock_camera_to_entity, GameLayer};
use bevy::{prelude::*, transform::TransformSystem};
use bevy_xpbd_3d::{math::*, prelude::*};
use character_controller::{CharacterControllerBundle, CharacterControllerPlugin};

mod character_controller;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterControllerPlugin)
            .add_systems(Startup, spawn_player)
            .add_systems(
                PostUpdate,
                lock_camera_to_entity::<Player>
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.4, 1.0)),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(
            Collider::capsule(1.0, 0.4),
            Vector::NEG_Y * 9.81,
            CollisionLayers::new(
                GameLayer::Player,
                [
                    GameLayer::Terrain,
                    GameLayer::Prop,
                    GameLayer::Projectile,
                    GameLayer::Player,
                    GameLayer::Mob,
                ],
            ),
        )
        .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
    ));
}

// fn update_player_position(
//     mut player_query: Query<(&mut Transform, &Player)>,
//     marker_query: Query<&Transform, (With<PositionMarker>, Without<Player>)>,
//     time: Res<Time>,
// ) {
//     let marker = marker_query.single();
//     let (mut player_transform, player) = player_query.single_mut();
//
//     let direction = marker.translation.xz() - player_transform.translation.xz();
//     let distance = direction.length_squared();
//
//     if distance >= 0.5 {
//         let target_rotation = (direction.x).atan2(direction.y);
//         player_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0., target_rotation, 0.);
//
//         let step_magnitude = MOVEMENT_SPEED * time.delta_seconds();
//         if step_magnitude.powi(2) > distance {
//             player_transform.translation = Transform::from_xyz(
//                 marker.translation.x,
//                 player.dimension.y * 0.5,
//                 marker.translation.z,
//             )
//             .translation;
//         } else {
//             let normalized = direction.normalize();
//             player_transform.translation += vec3(normalized.x, 0., normalized.y) * step_magnitude;
//         }
//     }
// }