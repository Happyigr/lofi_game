use bevy::prelude::*;

use crate::enemy::EnemyBundle;

pub fn spawn_enemies(mut commands: Commands) {
    (0..6).for_each(|_| {
        let (enemy, text) = EnemyBundle::new_random();
        commands.spawn(enemy).with_children(|parent| {
            parent.spawn(text);
        });
    })
}
