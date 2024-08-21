use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::resources::{Materials, MySettings};

pub fn start_bg_audio(materials: Res<Materials>, audio: Res<Audio>) {
    audio.play(materials.bg_sound.clone()).looped();
}

// this system will change the sound playing if the settings was changed
pub fn change_audio(settings: ResMut<MySettings>, audio: Res<Audio>) {
    if settings.audio_on {
        audio.resume();
        audio.set_volume(settings.bg_sound_volume);
    } else {
        audio.pause();
    }
}
