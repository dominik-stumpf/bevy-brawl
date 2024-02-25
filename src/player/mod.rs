use crate::{camera::utils::lock_camera_to_entity, prelude::*};
use ability_caster_controller::AbilityCasterControllerPlugin;
use bevy::transform::TransformSystem;
use bevy_xpbd_3d::{math::*, prelude::*};
use character_controller::{CharacterControllerBundle, CharacterControllerPlugin};

mod ability_caster_controller;
mod character_controller;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterControllerPlugin)
            .add_plugins(AbilityCasterControllerPlugin)
            .add_systems(Startup, spawn_player)
            .add_systems(
                PostUpdate,
                lock_camera_to_entity::<Player>
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.4, 1.0)),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(
            Collider::capsule(1.0, 0.4),
            Vector::NEG_Y * 9.81,
            CollisionLayers::new(
                GameLayer::Player,
                [
                    GameLayer::Terrain,
                    GameLayer::Prop,
                    GameLayer::Projectile,
                    GameLayer::Player,
                    GameLayer::Mob,
                ],
            ),
        )
        .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
    ));
}
