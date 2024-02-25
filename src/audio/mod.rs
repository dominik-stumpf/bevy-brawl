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
    pub use super::EventPlaySFX;
}

#[derive(Event)]
pub struct EventPlaySFX {
    pub audio_source: Handle<AudioSource>,
}

impl EventPlaySFX {
    pub fn new(audio_source: Handle<AudioSource>) -> Self {
        return Self { audio_source };
    }
}

// pub(crate) enum SFXKind {/* ... */}

fn play_sfx(audio: Res<Audio>, mut play_sfx_event: EventReader<EventPlaySFX>) {
    for play_event in play_sfx_event.read() {
        audio.play(play_event.audio_source.clone());
    }
}
