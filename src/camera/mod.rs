use self::skybox::Cubemap;
use crate::prelude::*;
use bevy::core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping, Skybox};

mod skybox;
pub mod utils;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (spawn_camera, spawn_light))
            .add_systems(
                Update,
                (animate_light_direction, skybox::asset_loaded).run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct MainCamera {
    pub initial_position: Vec3,
}

fn spawn_camera(mut commands: Commands, assets: Res<GameAssets>) {
    let initial_position = Vec3::new(0.0, 20.0, 12.0);
    let camera = Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        transform: Transform::from_xyz(initial_position.x, initial_position.y, initial_position.z)
            .looking_at(Vec3::ZERO, -Vec3::Z),
        ..default()
    };
    let skybox_handle = assets.environment_map.clone();

    commands.spawn((
        Name::new("MainCamera"),
        camera,
        MainCamera { initial_position },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 100.0,
        },
        EnvironmentMapLight {
            diffuse_map: assets.diffuse_map.clone(),
            specular_map: assets.specular_map.clone(),
            intensity: 500.0,
        },
        BloomSettings::default(),
    ));

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
    });
}

fn spawn_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 0.1,
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 1000.0,
            ..default()
        },
        ..default()
    });
}

pub fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.02);
    }
}
