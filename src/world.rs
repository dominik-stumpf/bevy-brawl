use crate::prelude::*;
use bevy::scene::SceneInstance;
use bevy_xpbd_3d::{
    components::{CollisionLayers, LayerMask, RigidBody},
    plugins::collision::{AsyncSceneCollider, Collider, ComputedCollider},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_world_map)
            .add_systems(
                Update,
                add_terrain_component_recursively
                    .run_if(any_with_component::<Terrain>)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct Terrain {
    is_terrain_added_recursively: bool,
}

fn spawn_world_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    commands.spawn((
        Name::new("WorldMap"),
        SceneBundle {
            scene: assets.world_map.clone(),
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
        Terrain {
            is_terrain_added_recursively: false,
        },
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

fn add_terrain_component_recursively(
    mut world_map_query: Query<(&SceneInstance, &mut Terrain)>,
    mut commands: Commands,
    collider_query: Query<Entity, With<Collider>>,
    scene_manager: Res<SceneSpawner>,
) {
    let Ok((instance, mut terrain)) = world_map_query.get_single_mut() else {
        return;
    };
    if terrain.is_terrain_added_recursively {
        info_once!("Loaded all terrain");
        return;
    }
    let colliders = collider_query.iter_many(scene_manager.iter_instance_entities(**instance));
    for collider_entity in colliders {
        commands.entity(collider_entity).insert(Terrain {
            is_terrain_added_recursively: true,
        });
        terrain.is_terrain_added_recursively = true;
    }
}
