use ability::AbilityPlugin;
use bevy::prelude::*;
use bevy_xpbd_3d::{plugins::PhysicsPlugins, prelude::PhysicsLayer};
use camera::CameraPlugin;
use cursor_caster::CursorCasterPlugin;
use debug::DebugPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod ability;
mod camera;
mod cursor_caster;
mod debug;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    name: Some("game-window".into()),
                    ..default()
                }),
                ..default()
            }),
            bevy_framepace::FramepacePlugin,
            DebugPlugin,
            WorldPlugin,
            CameraPlugin,
            PhysicsPlugins::default(),
            PlayerPlugin,
            CursorCasterPlugin,
            AbilityPlugin,
        ))
        .insert_resource(Msaa::Sample8)
        .run();
}

/// Collision layer for entities
#[derive(PhysicsLayer, Clone, Copy, Debug)]
pub enum GameLayer {
    /// Player controlled character
    Player,
    /// Mobile object: AI controlled character
    Mob,
    /// Part of the world map that has collider
    Terrain,
    /// Flying object propelled by exernal force
    Projectile,
    /// Destructible object
    Prop,
}
