use core::fmt::Display;

use crate::{constants::*, resources::Materials};
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use rand::Rng;
use strum::{EnumCount, EnumIter};

#[derive(Clone, Debug, PartialEq, Eq, EnumCount, EnumIter)]
pub enum SuperPower {
    Boost,
    Jump,
    CatchRad,
    Boom,
    Poop,
}

impl Display for SuperPower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuperPower::Boost => write!(f, "Boost"),
            SuperPower::Jump => write!(f, "Jump"),
            SuperPower::CatchRad => write!(f, "CatchRad"),
            SuperPower::Boom => write!(f, "Boom"),
            SuperPower::Poop => write!(f, "Poop"),
        }
    }
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

    pub fn get_sound(&self, materials: &Materials) -> Handle<AudioSource> {
        match self {
            SuperPower::Boost => materials.xylophone.c.clone(),
            SuperPower::Jump => materials.xylophone.d.clone(),
            SuperPower::CatchRad => materials.xylophone.e.clone(),
            SuperPower::Boom => materials.xylophone.f.clone(),
            SuperPower::Poop => materials.xylophone.g.clone(),
        }
    }
}
