use crate::prelude::*;
use bevy_kira_audio::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_kira_audio::prelude::AudioPlugin)
            .add_event::<EventPlaySFX>()
            .add_systems(Update, play_sfx.run_if(in_state(GameState::InGame)));
    }
}

pub mod prelude {
    pub use super::{EventPlaySFX, SFXKind};
}

#[derive(Event)]
pub struct EventPlaySFX {
    pub audio_source: SFXKind,
}

impl EventPlaySFX {
    pub fn new(audio_source: SFXKind) -> Self {
        return Self { audio_source };
    }
}

#[derive(Clone, Copy)]
pub enum SFXKind {
    MagicMissileExplosion,
    MagicMissileCast,
}

fn resolve_sfx_asset(sfx: SFXKind, assets: &Res<GameAssets>) -> Handle<AudioSource> {
    match sfx {
        SFXKind::MagicMissileCast => assets.missile_cast_sfx.clone(),
        SFXKind::MagicMissileExplosion => assets.missile_explosion_2_sfx.clone(),
    }
}

fn play_sfx(
    audio: Res<Audio>,
    mut play_sfx_event: EventReader<EventPlaySFX>,
    assets: Res<GameAssets>,
) {
    for play_event in play_sfx_event.read() {
        audio.play(resolve_sfx_asset(play_event.audio_source, &assets));
    }
}
