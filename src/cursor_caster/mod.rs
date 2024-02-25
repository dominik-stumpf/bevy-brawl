use crate::{camera::MainCamera, prelude::*};
use bevy::window::PrimaryWindow;
use bevy_xpbd_3d::{math::*, prelude::*};

pub struct CursorCasterPlugin;

impl Plugin for CursorCasterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition::default())
            .add_systems(OnEnter(GameState::InGame), spawn_position_marker)
            .add_systems(
                Update,
                (
                    draw_position_marker_gizmo,
                    update_cursor_position,
                    update_marker_position,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Resource, Default)]
pub struct CursorPosition(pub Vec3);

fn update_cursor_position(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_position_resource: ResMut<CursorPosition>,
    mut gizmos: Gizmos,
    spatial_query: SpatialQuery,
) {
    let Ok(camera_result) = camera_query.get_single() else {
        warn_once!("MainCamera was not found");
        return;
    };

    let Some(cursor_position) = window_query.single().cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera_result;
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let origin = camera_transform.translation();
    let direction = ray.direction;

    if let Some(ray_hit_data) = spatial_query.cast_ray(
        origin,
        direction,
        Scalar::MAX,
        true,
        SpatialQueryFilter::from_mask(GameLayer::Terrain),
    ) {
        let contact_point = origin + direction.adjust_precision() * ray_hit_data.time_of_impact;
        cursor_position_resource.0 = contact_point;

        gizmos.circle(
            contact_point,
            Direction3d::new_unchecked(Vec3::Y),
            0.3,
            Color::MIDNIGHT_BLUE,
        );
    }
}

#[derive(Component)]
pub struct PositionMarker;

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
