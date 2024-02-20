use bevy::{math::vec3, prelude::*};

use crate::{camera::MainCamera, CursorPosition};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_position_marker))
            .add_systems(
                Update,
                (
                    update_marker_position,
                    draw_position_marker_gizmo,
                    update_player_position,
                ),
            );
    }
}

#[derive(Component)]
pub struct Player {
    dimension: Vec3,
}

#[derive(Component)]
struct PositionMarker;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_dimension = vec3(1.0, 2.0, 1.0);
    let player = PbrBundle {
        mesh: meshes.add(Cuboid::new(
            player_dimension.x,
            player_dimension.y,
            player_dimension.z,
        )),
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(0.0, player_dimension.y * 0.5, 0.0),
        ..default()
    };

    commands.spawn((
        player,
        Player {
            dimension: player_dimension,
        },
    ));
}

fn update_player_position(
    mut player_query: Query<(&mut Transform, &Player)>,
    marker_query: Query<&Transform, (With<PositionMarker>, Without<Player>)>,
    time: Res<Time>,
) {
    let marker = marker_query.single();
    let (mut player_transform, player) = player_query.single_mut();

    let direction = marker.translation.xz() - player_transform.translation.xz();
    let distance = direction.length_squared();

    if distance >= 0.5 {
        let target_rotation = (direction.x).atan2(direction.y);
        player_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0., target_rotation, 0.);

        let step_magnitude = 5.0 * time.delta_seconds();
        if step_magnitude.powi(2) > distance {
            player_transform.translation = Transform::from_xyz(
                marker.translation.x,
                player.dimension.y * 0.5,
                marker.translation.z,
            )
            .translation;
        } else {
            let normalized = direction.normalize();
            player_transform.translation += vec3(normalized.x, 0., normalized.y) * step_magnitude;
        }
    }
}

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

// const MAIN_CAMERA_TRANSFORM_OFFSET: Vec3 = Vec3::new(0., 70., -70.);
// fn lock_camera_to_entity<T: Component>(
//     mut param_query: ParamSet<(
//         Query<&Transform, With<T>>,
//         Query<&mut Transform, With<MainCamera>>,
//     )>,
// ) {
//     let mut target_translation = Vec3::ZERO;
//     for target in param_query.p0().iter_mut() {
//         target_translation = target.translation;
//     }
//
//     for mut camera in param_query.p1().iter_mut() {
//         camera.translation =
//             Transform::from_translation(target_translation + MAIN_CAMERA_TRANSFORM_OFFSET)
//                 .looking_at(target_translation, Vec3::Z)
//                 .translation;
//     }
// }
