use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::resources::{Game, Materials};

pub fn spawn_heartbeat_bg_sound(
    mut commands: Commands,
    materials: Res<Materials>,
    audio: Res<Audio>,
    game: Res<Game>,
) {
    audio.play(materials.heartbeat_bg_sound.clone()).looped();
    audio.set_playback_rate(game.settings.heartbeat_speed);
}
