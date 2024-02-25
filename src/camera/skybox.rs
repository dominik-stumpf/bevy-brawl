use crate::prelude::*;
use bevy::{
    core_pipeline::Skybox,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

#[derive(Resource)]
pub struct Cubemap {
    pub is_loaded: bool,
    pub image_handle: Handle<Image>,
}

pub fn asset_loaded(
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    assets: Res<GameAssets>,
    mut skyboxes: Query<&mut Skybox>,
) {
    let image = images.get_mut(&assets.environment_map).unwrap();
    // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
    // so they appear as one texture. The following code reconfigures the texture as necessary.
    if image.texture_descriptor.array_layer_count() == 1 {
        image.reinterpret_stacked_2d_as_array(image.height() / image.width());
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }

    for mut skybox in &mut skyboxes {
        skybox.image = cubemap.image_handle.clone();
    }

    cubemap.is_loaded = true;
}
