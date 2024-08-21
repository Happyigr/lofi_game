use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::resources::{Materials, MySettings};

// this system will change the sound playing if the settings was changed
pub fn change_audio(materials: Res<Materials>, settings: ResMut<MySettings>, audio: Res<Audio>) {
    if settings.audio_on {
        if audio.is_playing_sound() == false {
            audio.play(materials.heartbeat_bg_sound.clone()).looped();
        }
        audio.resume();
        audio.set_playback_rate(settings.heartbeat_speed);
    } else {
        audio.stop();
    }
}
