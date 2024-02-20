use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
pub struct MainCamera {
    pub initial_position: Vec3,
}

fn spawn_camera(mut commands: Commands) {
    let initial_position = Vec3::new(15.0, 5.0, 15.0);
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(initial_position.x, initial_position.y, initial_position.z)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn((camera, MainCamera { initial_position }));
}
