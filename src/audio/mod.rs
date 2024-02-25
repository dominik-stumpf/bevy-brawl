use crate::prelude::*;
use bevy_kira_audio::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_kira_audio::prelude::AudioPlugin)
            .add_systems(OnEnter(GameState::InGame), play_loop);
    }
}

fn play_loop(assets: Res<GameAssets>, audio: Res<Audio>) {
    audio.play(assets.missile_cast_sfx.clone());
}
