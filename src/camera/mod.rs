use self::skybox::{Cubemap, CUBEMAPS};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping, Skybox},
    prelude::*,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

mod skybox;
pub mod utils;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            // .add_systems(Startup, spawn_panorbit_camera)
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    skybox::cycle_cubemap_asset,
                    skybox::asset_loaded.after(skybox::cycle_cubemap_asset),
                ),
            );
    }
}

#[derive(Component)]
pub struct MainCamera {
    pub initial_position: Vec3,
}

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    let skybox_handle = asset_server.load(CUBEMAPS[0].0);

    commands.spawn((
        Name::new("MainCamera"),
        camera,
        MainCamera { initial_position },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 300.0,
        },
        EnvironmentMapLight {
            diffuse_map: asset_server
                .load("environment_maps/kloofendal_43d_clear_puresky_diff_1k.ktx2"),
            specular_map: asset_server
                .load("environment_maps/kloofendal_43d_clear_puresky_spec_1k.ktx2"),
            intensity: 1200.0,
        },
        BloomSettings::default(),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 0.2,
    });

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}

fn spawn_panorbit_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox_handle = asset_server.load(CUBEMAPS[0].0);

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
        Skybox {
            image: skybox_handle.clone(),
            brightness: 150.0,
        },
        EnvironmentMapLight {
            diffuse_map: asset_server
                .load("environment_maps/kloofendal_43d_clear_puresky_diff_1k.ktx2"),
            specular_map: asset_server
                .load("environment_maps/kloofendal_43d_clear_puresky_spec_1k.ktx2"),
            intensity: 1200.0,
        },
    ));

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}
