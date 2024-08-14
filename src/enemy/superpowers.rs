use crate::constants::*;
use bevy::prelude::*;
use rand::Rng;
use strum::EnumCount;

#[derive(Clone, Debug, PartialEq, Eq, EnumCount)]
pub enum SuperPower {
    Boost,
    Jump,
    CatchRad,
    Boom,
    Poop,
}

impl SuperPower {
    pub fn rand_new() -> SuperPower {
        let num: u8 = rand::thread_rng().gen_range(0..SuperPower::COUNT as u8);

        match num {
            0 => SuperPower::Boost,
            1 => SuperPower::Jump,
            2 => SuperPower::CatchRad,
            3 => SuperPower::Boom,
            4 => SuperPower::Poop,
            _ => unreachable!(),
        }
    }

    pub fn get_enemy_color(&self) -> Color {
        match self {
            SuperPower::Boost => ENEMY_COLOR_BOOST,
            SuperPower::Jump => ENEMY_COLOR_JUMP,
            SuperPower::CatchRad => ENEMY_COLOR_CATCHRAD,
            SuperPower::Boom => ENEMY_COLOR_BOOM,
            SuperPower::Poop => ENEMY_COLOR_POOP,
        }
    }

    pub fn get_keycode(&self) -> KeyCode {
        match self {
            SuperPower::Boost => BOOST_ACTIVATOR,
            SuperPower::Jump => JUMP_ACTIVATOR,
            SuperPower::CatchRad => CATCHRAD_ACTIVATOR,
            SuperPower::Boom => BOOM_ACTIVATOR,
            SuperPower::Poop => POOP_ACTIVATOR,
        }
    }

    pub fn get_keycode_str(&self) -> &str {
        match self {
            SuperPower::Boost => "1",
            SuperPower::Jump => "2",
            SuperPower::CatchRad => "3",
            SuperPower::Boom => "4",
            SuperPower::Poop => "5",
        }
    }
}
