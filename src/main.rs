use ability::AbilityPlugin;
use bevy::prelude::*;
use bevy_xpbd_3d::plugins::PhysicsPlugins;
use camera::CameraPlugin;
use cursor_caster::CursorCasterPlugin;
use debug::DebugPlugin;
use player::PlayerPlugin;
use prelude::GameControlsPlugin;
use world::WorldPlugin;

mod ability;
mod camera;
mod cursor_caster;
mod debug;
mod game_controls;
mod player;
mod prelude;
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
            GameControlsPlugin,
        ))
        .insert_resource(Msaa::Sample8)
        .run();
}
