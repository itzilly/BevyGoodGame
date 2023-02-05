use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

#[derive(Component, Clone, Default)]
pub struct Health(f32);

#[derive(Component, Clone, Default)]
pub struct AttackPower(f32);

#[derive(Component, Clone, Default)]
pub struct DamageResistance(f32);

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
            health: Health(10.0),
            attack_power: AttackPower(10.0),
            damage_resistance: DamageResistance(10.0),
            player_movement_info: PlayerMovementInfo {
                acceleration: 800.0,
                deceleration: 700.0,
                max_speed: 200.0,
            },
            attack_duration: AttackTimer(Timer::new(
                Duration::from_millis((0.5 * 1000.0) as u64),
                TimerMode::Repeating,
            )),
        }
    }
}

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
            _ => ColliderBundle::default(),
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
