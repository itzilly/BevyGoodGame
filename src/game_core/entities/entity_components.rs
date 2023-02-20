use crate::game_core::entities::player::{
    ATTACK_DURATION_SECS, ATTACK_POWER, DAMAGE_RESISTANCE, HEALTH, PLAYER_MOVEMENT,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::cell::RefMut;
use std::os::macos::raw::stat;
use std::time::Duration;

// COMMON COMPONENTS
#[derive(Component, Clone, Default)]
pub struct Health(f32);

#[derive(Component, Clone, Default)]
pub struct AttackPower(f32);

#[derive(Component, Clone, Default)]
pub struct DamageResistance(f32);

#[derive(Clone, Debug, Default, Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(15.0 / 2.0, 22.0 / 2.0),
                rigid_body: RigidBody::Dynamic,
                velocity: Velocity::zero(),
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                gravity_scale: GravityScale(0.0),
            },
            "Enemy" => ColliderBundle {
                collider: Collider::cuboid(16.0 / 2.0, 12.0 / 2.0),
                rigid_body: RigidBody::Fixed,
                velocity: Velocity::zero(),
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                gravity_scale: GravityScale(0.0),
            },
            _ => ColliderBundle::default(),
        }
    }
}

// PLAYER COMPONENTS
#[derive(Component, Clone, Default)]
pub struct Player;

#[derive(Component, Clone, Default)]
pub struct AttackTimer(pub Timer);

#[derive(Component, Clone, Default)]
pub struct IsAttacking(pub bool);

#[derive(Component, Clone, Default)]
pub struct PlayerMovementInfo {
    pub acceleration: f32,
    pub deceleration: f32,
    pub max_speed: f32,
}

#[derive(Bundle, Clone, Default)]
pub struct PlayerStatsBundle {
    pub health: Health,
    pub attack_power: AttackPower,
    pub damage_resistance: DamageResistance,
    pub player_movement_info: PlayerMovementInfo,
    pub attack_duration: AttackTimer,
}

impl From<EntityInstance> for PlayerStatsBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        PlayerStatsBundle {
            health: Health(HEALTH),
            attack_power: AttackPower(ATTACK_POWER),
            damage_resistance: DamageResistance(DAMAGE_RESISTANCE),
            player_movement_info: PLAYER_MOVEMENT.clone(),
            attack_duration: AttackTimer(Timer::new(
                Duration::from_millis((ATTACK_DURATION_SECS * 1000.0) as u64),
                TimerMode::Repeating,
            )),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("tile_sets/mystic_woods_free_2.1/sprites/characters/player_sprite.png")]
    #[bundle]
    pub sprite_bundle: SpriteBundle,

    #[from_entity_instance]
    #[bundle]
    player_stats_bundle: PlayerStatsBundle,

    #[from_entity_instance]
    #[bundle]
    collider_bundle: ColliderBundle,
    player: Player,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    is_attacking: IsAttacking,

    #[worldly]
    worldly: Worldly,
}

// ENEMY COMPONENTS
#[derive(Component, Clone, Default)]
pub struct Enemy {
    pub name: String,
    pub stats: EnemyStats,
}

impl Enemy {
    // Return elements are (attacking back, died)
    pub fn attack(&self, incoming_damage: f32, mut enemy_stats: &mut Mut<EnemyStats>)
    /*-> (bool, bool)*/
    {
        enemy_stats.health -= incoming_damage;
        let mut dead = false;
        if enemy_stats.health <= 0.0 {
            dead = true;
            enemy_stats.health = self.stats.health;
            println!("DEAD!!!");
        }
        println!("Health is now: {}", enemy_stats.health);
    }
}

impl From<EntityInstance> for Enemy {
    fn from(entity_instance: EntityInstance) -> Self {
        let mut enemy = Enemy::default();
        for field in entity_instance.field_instances {
            match field.identifier.as_ref() {
                "Name" => {
                    if let FieldValue::String(Some(name)) = field.value {
                        enemy.name = name;
                    }
                }
                "Health" => {
                    if let FieldValue::Float(Some(health)) = field.value {
                        enemy.stats.health = health;
                    }
                }
                "Attack_Power" => {
                    if let FieldValue::Float(Some(power)) = field.value {
                        enemy.stats.attack_power = power;
                    }
                }
                "Attack_Chance" => {
                    if let FieldValue::Float(Some(chance)) = field.value {
                        enemy.stats.attack_chance = chance;
                    }
                }
                _ => {}
            }
        }

        return enemy;
    }
}

// Chance of attacking back, between 0 and 100
#[derive(Component, Clone, Default)]
pub struct AttackChance(f32);

#[derive(Component, Clone, Default)]
pub struct EnemyStats {
    pub health: f32,
    pub attack_power: f32,
    pub attack_chance: f32,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    #[sprite_bundle("tile_sets/mystic_woods_free_2.1/sprites/characters/slime_sprite.png")]
    #[bundle]
    pub sprite_bundle: SpriteBundle,

    #[from_entity_instance]
    pub enemy: Enemy,

    pub enemy_current_stats: EnemyStats,

    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}
