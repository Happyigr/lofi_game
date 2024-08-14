use crate::constants::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub up_key: KeyCode,
    pub down_key: KeyCode,
    pub right_key: KeyCode,
    pub left_key: KeyCode,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            up_key: PLAYER_UP,
            down_key: PLAYER_DOWN,
            right_key: PLAYER_RIGHT,
            left_key: PLAYER_LEFT,
        }
    }
}
