use crate::constants::*;
use std::time::Duration;

use rand::Rng;

use bevy::prelude::*;
use bevy::time::{Timer, TimerMode};

#[derive(Component)]
pub struct BoomAnim;

#[derive(Component)]
pub struct AnimConfig {
    pub first_sprite_i: usize,
    pub last_sprite_i: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_i: first,
            last_sprite_i: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

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
            up_key: KeyCode::KeyW,
            down_key: KeyCode::KeyS,
            right_key: KeyCode::KeyA,
            left_key: KeyCode::KeyD,
        }
    }
}

#[derive(Component)]
pub struct Catchable;

#[derive(Component)]
pub struct EnemyToDespawn;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SuperPower {
    Boost,
    Jump,
    CatchRad,
    Boom,
}

impl SuperPower {
    pub fn rand_new() -> SuperPower {
        let num: u8 = rand::thread_rng().gen_range(0..3);

        match num {
            0 => SuperPower::Boost,
            1 => SuperPower::Jump,
            2 => SuperPower::CatchRad,
            3 => SuperPower::Boom,
            _ => unreachable!(),
        }
    }

    pub fn get_enemy_color(&self) -> Color {
        match self {
            SuperPower::Boost => ENEMY_COLOR_BOOST,
            SuperPower::Jump => ENEMY_COLOR_JUMP,
            SuperPower::CatchRad => ENEMY_COLOR_CATCHRAD,
            SuperPower::Boom => ENEMY_COLOR_BOOM,
        }
    }

    pub fn get_keycode(&self) -> KeyCode {
        match self {
            SuperPower::Boost => BOOST_ACTIVATOR,
            SuperPower::Jump => JUMP_ACTIVATOR,
            SuperPower::CatchRad => CATCHRAD_ACTIVATOR,
            SuperPower::Boom => BOOM_ACTIVATOR,
        }
    }

    pub fn get_keycode_str(&self) -> &str {
        match self {
            SuperPower::Boost => "1",
            SuperPower::Jump => "2",
            SuperPower::CatchRad => "3",
            SuperPower::Boom => "4",
        }
    }
}
#[derive(Component)]
pub struct Enemy {
    pub super_power: SuperPower,
}
