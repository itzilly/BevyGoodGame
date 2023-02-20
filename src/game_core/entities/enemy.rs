use super::entity_components::Enemy;
use crate::game_core::entities::entity_components::{EnemyBundle, EnemyStats, Player};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn on_enemy_spawned(mut enemies: Query<(&Enemy, &mut EnemyStats), Added<Enemy>>) {
    for (enemy, mut current_stats) in enemies.iter_mut() {
        current_stats.health = enemy.stats.health;
        current_stats.attack_chance = enemy.stats.attack_chance;
        current_stats.attack_power = enemy.stats.attack_power;
    }
}
