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

    #[asset(path = "audio/560575__theplax__explosion-4.ogg")]
    pub missile_explosion_1_sfx: Handle<AudioSource>,
    #[asset(path = "audio/560576__theplax__explosion-3.ogg")]
    pub missile_explosion_2_sfx: Handle<AudioSource>,
    #[asset(path = "audio/560578__theplax__explosion-6.ogg")]
    pub missile_explosion_3_sfx: Handle<AudioSource>,
    #[asset(path = "audio/magic-smite-6012.ogg")]
    pub missile_cast_sfx: Handle<AudioSource>,
}
