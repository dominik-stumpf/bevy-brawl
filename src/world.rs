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
) {
    let ground = PbrBundle {
        mesh: meshes.add(Circle::new(16.0)),
        material: materials.add(Color::YELLOW_GREEN),
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
            illuminance: 500.,
            ..default()
        },
        ..default()
    };

    commands.spawn(directional_light);
    commands.spawn(point_light);
}
