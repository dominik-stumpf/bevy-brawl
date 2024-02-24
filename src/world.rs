use crate::GameLayer;
use bevy::prelude::*;
use bevy_xpbd_3d::{
    components::{CollisionLayers, LayerMask, RigidBody},
    plugins::collision::{AsyncSceneCollider, Collider, ComputedCollider},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world_map);
    }
}

#[derive(Component)]
pub struct Terrain;

fn spawn_world_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("WorldMap"),
        SceneBundle {
            scene: asset_server.load("models/test_map.glb#Scene0"),
            ..default()
        },
        AsyncSceneCollider::new(Some(ComputedCollider::ConvexHull)).with_layers_for_name(
            "Terrain",
            CollisionLayers::new(
                GameLayer::Terrain,
                [GameLayer::Player, GameLayer::Projectile],
            ),
        ),
        RigidBody::Static,
        Terrain,
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::sphere(1.0),
        CollisionLayers::new(GameLayer::Prop, LayerMask::ALL),
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.0)),
            material: materials.add(StandardMaterial {
                emissive: Color::PURPLE * 500.0,
                ..default()
            }),
            transform: Transform::from_xyz(-5.0, 9.0, -5.0),
            ..default()
        },
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        CollisionLayers::new(GameLayer::Prop, LayerMask::ALL),
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(3.0, 2.0, 3.0).with_scale(Vec3::splat(3.0)),
            ..default()
        },
    ));
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
            illuminance: 500.0,
            ..default()
        },
        ..default()
    };

    commands.spawn(directional_light);
    commands.spawn(point_light);
}
