use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground, spawn_light))
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.2,
            });
    }
}

#[derive(Component)]
pub struct Ground;

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let ground_material = StandardMaterial {
        normal_map_texture: Some(asset_server.load("textures/coast_sand_rocks_02_nor_gl_1k.jpg")),
        base_color_texture: Some(asset_server.load("textures/coast_sand_rocks_02_diff_1k.jpg")),
        occlusion_texture: Some(asset_server.load("textures/coast_sand_rocks_02_ao_1k.jpg")),
        depth_map: Some(asset_server.load("textures/coast_sand_rocks_02_disp_1k.jpg")),
        perceptual_roughness: 0.8,
        reflectance: 0.5,
        parallax_depth_scale: 0.04,
        parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 8 },
        ..default()
    };

    let ground_mesh = Mesh::from(Circle::new(16.0))
        .with_generated_tangents()
        .expect("generate tangets for normal map");

    let ground = PbrBundle {
        mesh: meshes.add(ground_mesh),
        material: materials.add(ground_material),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    };

    commands.spawn((ground, Ground));
}

fn spawn_light(mut commands: Commands) {
    let point_light = PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4., 4., 4.),
        ..default()
    };

    let directional_light = DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 1200.,
            ..default()
        },
        ..default()
    };

    commands.spawn(directional_light);
    commands.spawn(point_light);
}