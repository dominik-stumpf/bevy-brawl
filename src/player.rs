use bevy::prelude::*;

use crate::{camera::MainCamera, CursorPosition};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_position_marker))
            .add_systems(Update, (update_marker_position, draw_position_marker_gizmo));
    }
}

#[derive(Component)]
pub struct Mage;

#[derive(Component)]
struct PositionMarker;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 2.0, 1.0)),
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    };

    commands.spawn((player, Mage));
}

fn spawn_position_marker(mut commands: Commands) {
    commands.spawn((PositionMarker, Transform::default()));
}

fn draw_position_marker_gizmo(mut gizmos: Gizmos, query: Query<&Transform, With<PositionMarker>>) {
    for transform in query.iter() {
        gizmos.circle(
            transform.translation,
            Direction3d::new_unchecked(*transform.up()), // Up vector is already normalized.
            0.15,
            Color::RED,
        );
    }
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
