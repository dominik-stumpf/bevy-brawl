use bevy::prelude::*;
use camera::{CameraPlugin, MainCamera};
use debug::DebugPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod camera;
mod debug;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_framepace::FramepacePlugin,
            DebugPlugin,
            WorldPlugin,
            CameraPlugin,
            PlayerPlugin,
        ))
        .insert_resource(CursorPosition::default())
        .add_systems(Update, update_cursor_position)
        .run();
}

#[derive(Resource, Default)]
pub struct CursorPosition(Vec3);

fn update_cursor_position(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Query<&Window>,
    mut cursor_position_resource: ResMut<CursorPosition>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let plane = Transform::from_xyz(0., 0., 0.);

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(plane.translation, Plane3d { normal: plane.up() }) else {
        return;
    };
    let point = ray.get_point(distance);

    cursor_position_resource.0 = point + plane.up() * 0.01;

    gizmos.circle(
        point + plane.up() * 0.01,
        Direction3d::new_unchecked(*plane.up()), // Up vector is already normalized.
        0.2,
        Color::WHITE,
    );
}
