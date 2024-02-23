use bevy::{prelude::*, transform::TransformSystem};
use bevy_xpbd_3d::{math::*, prelude::*};

use crate::{
    camera::MainCamera, character_controller::CharacterControllerBundle,
    cursor_caster::CursorPosition, GameLayer,
};

const MOVEMENT_SPEED: f32 = 8.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_position_marker))
            .add_systems(Update, (update_marker_position, draw_position_marker_gizmo))
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

#[derive(Component)]
struct PositionMarker;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
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
            CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
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

fn spawn_position_marker(mut commands: Commands) {
    commands.spawn((PositionMarker, Transform::default()));
}

fn draw_position_marker_gizmo(
    mut gizmos: Gizmos,
    marker_query: Query<&Transform, With<PositionMarker>>,
) {
    let transform = marker_query.single();
    gizmos.circle(
        transform.translation,
        Direction3d::new_unchecked(*transform.up()), // Up vector is already normalized.
        0.15,
        Color::RED,
    );
}

fn update_marker_position(
    mut marker_query: Query<&mut Transform, With<PositionMarker>>,
    cursor_position_query: Res<CursorPosition>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    let mut transform = marker_query.single_mut();

    if mouse_input.pressed(MouseButton::Left) {
        transform.translation = cursor_position_query.0;
    }
}

fn lock_camera_to_entity<T: Component>(
    mut param_query: ParamSet<(
        Query<&Transform, With<T>>,
        Query<(&mut Transform, &MainCamera)>,
    )>,
) {
    let mut target_translation = Vec3::ZERO;
    for target in param_query.p0().iter_mut() {
        target_translation = target.translation;
    }

    for (mut camera_transform, camera) in param_query.p1().iter_mut() {
        camera_transform.translation =
            Transform::from_translation(target_translation + camera.initial_position)
                .looking_at(target_translation, Vec3::Z)
                .translation;
    }
}
