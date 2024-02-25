use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "environment_maps/kloofendal_43d_clear_puresky_1k.png")]
    pub environment_map: Handle<Image>,
    #[asset(path = "environment_maps/kloofendal_43d_clear_puresky_diff_1k.ktx2")]
    pub diffuse_map: Handle<Image>,
    #[asset(path = "environment_maps/kloofendal_43d_clear_puresky_spec_1k.ktx2")]
    pub specular_map: Handle<Image>,
    #[asset(path = "models/test_map.glb#Scene0")]
    pub world_map: Handle<Scene>,
}
