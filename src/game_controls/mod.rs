use self::assets::GameAssets;
use crate::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::prelude::PhysicsLayer;

pub mod assets;
pub mod cleanup;
// pub mod prelude {
//     pub use super::assets::GameAssets;
//     pub use super::cleanup::*;
// }

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    AssetLoading,
    MainMenu,
    InGame,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum PlayingState {
    Playing,
    Paused,
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

pub struct GameControlsPlugin;

impl Plugin for GameControlsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(PlayingState::Playing)
            .insert_state(GameState::AssetLoading)
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::InGame)
                    .load_collection::<GameAssets>(),
            );
    }
}
