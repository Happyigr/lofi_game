use core::fmt::Display;

use crate::{constants::*, resources::Materials};
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use rand::Rng;
use strum::{EnumCount, EnumIter};

const SUPERPOWERS_AMOUNT: u8 = 5;
const NOTES_AMOUNT: u8 = 8;

#[derive(Clone, Debug, PartialEq, Eq, EnumCount, EnumIter)]
pub enum SuperPower {
    Boost,
    Jump,
    CatchRad,
    Boom,
    Poop,
    C,
    D,
    E,
    F,
    G,
    GM,
    A,
    B,
}

impl Display for SuperPower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuperPower::Boost => write!(f, "Boost"),
            SuperPower::Jump => write!(f, "Jump"),
            SuperPower::CatchRad => write!(f, "CatchRad"),
            SuperPower::Boom => write!(f, "Boom"),
            SuperPower::Poop => write!(f, "Poop"),
            SuperPower::C => write!(f, "C"),
            SuperPower::D => write!(f, "D"),
            SuperPower::E => write!(f, "E"),
            SuperPower::F => write!(f, "F"),
            SuperPower::G => write!(f, "G"),
            SuperPower::GM => write!(f, "GM"),
            SuperPower::A => write!(f, "A"),
            SuperPower::B => write!(f, "B"),
        }
    }
}

impl SuperPower {
    pub fn rand_new() -> SuperPower {
        let num = rand::thread_rng().gen_range(0.0..1.);

        match num {
            num if num <= 0.2 => SuperPower::rand_new_superpower(),
            _ => SuperPower::rand_new_note(),
        }
    }

    fn rand_new_note() -> SuperPower {
        let num: u8 = rand::thread_rng().gen_range(0..NOTES_AMOUNT);

        match num {
            0 => SuperPower::C,
            1 => SuperPower::D,
            2 => SuperPower::E,
            3 => SuperPower::F,
            4 => SuperPower::G,
            5 => SuperPower::GM,
            6 => SuperPower::A,
            7 => SuperPower::B,
            _ => panic!("Add new entry in rand_new_note of superpowers"),
        }
    }
    fn rand_new_superpower() -> SuperPower {
        let num: u8 = rand::thread_rng().gen_range(0..SUPERPOWERS_AMOUNT);

        match num {
            0 => SuperPower::Boost,
            1 => SuperPower::Jump,
            2 => SuperPower::CatchRad,
            3 => SuperPower::Boom,
            4 => SuperPower::Poop,
            _ => panic!("Add new entry in rand_new_superpower of superpowers"),
        }
    }

    pub fn get_enemy_color(&self) -> Color {
        match self {
            SuperPower::Boost => ENEMY_COLOR_BOOST,
            SuperPower::Jump => ENEMY_COLOR_JUMP,
            SuperPower::CatchRad => ENEMY_COLOR_CATCHRAD,
            SuperPower::Boom => ENEMY_COLOR_BOOM,
            SuperPower::Poop => ENEMY_COLOR_POOP,
            _ => NOTE_COLOR,
        }
    }

    pub fn get_keycode(&self) -> KeyCode {
        match self {
            SuperPower::Boost => BOOST_ACTIVATOR,
            SuperPower::Jump => JUMP_ACTIVATOR,
            SuperPower::CatchRad => CATCHRAD_ACTIVATOR,
            SuperPower::Boom => BOOM_ACTIVATOR,
            SuperPower::Poop => POOP_ACTIVATOR,
            SuperPower::C => C_ACTIVATOR,
            SuperPower::D => D_ACTIVATOR,
            SuperPower::E => E_ACTIVATOR,
            SuperPower::F => F_ACTIVATOR,
            SuperPower::G => G_ACTIVATOR,
            SuperPower::GM => GM_ACTIVATOR,
            SuperPower::A => A_ACTIVATOR,
            SuperPower::B => B_ACTIVATOR,
        }
    }

    pub fn get_keycode_str(&self) -> &str {
        match self {
            SuperPower::Boost => "1",
            SuperPower::Jump => "2",
            SuperPower::CatchRad => "3",
            SuperPower::Boom => "4",
            SuperPower::Poop => "5",
            SuperPower::C => "C",
            SuperPower::D => "D",
            SuperPower::E => "E",
            SuperPower::F => "F",
            SuperPower::G => "G",
            SuperPower::GM => "GM",
            SuperPower::A => "A",
            SuperPower::B => "B",
        }
    }

    pub fn get_sound(&self, materials: &Materials) -> Handle<AudioSource> {
        match self {
            SuperPower::C => materials.xylophone.c.clone(),
            SuperPower::D => materials.xylophone.d.clone(),
            SuperPower::E => materials.xylophone.e.clone(),
            SuperPower::F => materials.xylophone.f.clone(),
            SuperPower::G => materials.xylophone.g.clone(),
            SuperPower::GM => materials.xylophone.gm.clone(),
            SuperPower::A => materials.xylophone.a.clone(),
            SuperPower::B => materials.xylophone.b.clone(),
            _ => materials.heartbeat.clone(),
        }
    }
}
