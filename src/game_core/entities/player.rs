use crate::game_core::entities::entity_components::{
    AttackPower, AttackTimer, DamageResistance, Enemy, EnemyStats, Health, IsAttacking, Player,
    PlayerMovementInfo,
};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::egui::Event::Key;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::geometry::CollisionEventFlags;
use std::cell::RefMut;

use super::entity_components;

pub static HEALTH: f32 = 10.0;
pub static ATTACK_POWER: f32 = 10.0;
pub static DAMAGE_RESISTANCE: f32 = 10.0;
pub static PLAYER_MOVEMENT: PlayerMovementInfo = PlayerMovementInfo {
    acceleration: 800.0,
    deceleration: 700.0,
    max_speed: 200.0,
};
pub static ATTACK_DURATION_SECS: f32 = 0.1;

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Velocity, &PlayerMovementInfo, &Transform), With<Player>>,
) {
    // Get player information
    if let Ok((mut player_velocity, movement, transform)) = player.get_single_mut() {
        // If any movement keys pressed
        if keyboard_input.any_pressed([
            KeyCode::Up,
            KeyCode::W,
            KeyCode::Left,
            KeyCode::A,
            KeyCode::Down,
            KeyCode::S,
            KeyCode::Right,
            KeyCode::D,
        ]) {
            // Acceleration is the rate at which the speed increases
            let mut acceleration = Vec2::ZERO;

            // set the acceleration based on inputs
            if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                acceleration += Vec2::new(0.0, 1.0) * movement.acceleration;
            }
            if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                acceleration += Vec2::new(1.0, 0.0) * movement.acceleration;
            }
            if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                acceleration += Vec2::new(0.0, -1.0) * movement.acceleration;
            }
            if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                acceleration += Vec2::new(-1.0, 0.0) * movement.acceleration;
            }

            // Only if the acceleration is zero (ONLY IN CASES WHERE ALL ARE PRESSED)
            if acceleration.length_squared() == 0.0 {
                return;
            }

            // making sure the diagonals arent faster than they should be
            acceleration = acceleration.normalize();

            let velocity_dot_acceleration = player_velocity
                .linvel
                .normalize()
                .dot(acceleration.normalize());

            // When changing directions, accelerate more to make the turn faster
            if velocity_dot_acceleration < 0.9 {
                acceleration = acceleration * 4.0;
            } else if velocity_dot_acceleration > 0.9 && velocity_dot_acceleration < 0.98 {
                acceleration = (acceleration.normalize()
                    - (player_velocity.linvel.normalize() * 0.8))
                    .normalize()
            }

            // apply the velocity
            player_velocity.linvel += acceleration * movement.acceleration * time.delta_seconds();

            // Cap speed if beyond max
            if player_velocity.linvel.length() > movement.max_speed {
                player_velocity.linvel = player_velocity.linvel.normalize() * movement.max_speed;
            }
            // If no input AND if after this loop the velocity will be greater than zero
        } else if player_velocity.linvel.length() - (movement.deceleration * time.delta_seconds())
            > 0.0
        {
            // Decelerate
            player_velocity.linvel = player_velocity.linvel
                - player_velocity.linvel.normalize() * movement.deceleration * time.delta_seconds();
            return;
        } else {
            // Stop the player
            player_velocity.linvel = Vec2::ZERO;
            return;
        }
    }
}

pub fn attack_handler_system(
    mut commands: Commands,
    mut player: Query<
        (
            &mut IsAttacking,
            &Transform,
            &mut AttackTimer,
            Option<&Children>,
            Entity,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse: Res<Input<MouseButton>>,
) {
    if let Ok((mut attacking, transform, mut attack_timer, children, entity)) =
        player.get_single_mut()
    {
        if mouse.just_pressed(MouseButton::Left) && !attacking.0 {
            attacking.0 = true;
            if attacking.0 {
                let collider = commands
                    .spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Quad::new(Vec2::new(40.0, 40.0))))
                                .into(),
                            material: materials.add(ColorMaterial::from(Color::BLUE)),
                            ..default()
                        },
                        Collider::cuboid(20.0, 20.0),
                        ActiveEvents::COLLISION_EVENTS,
                        Sensor,
                    ))
                    .id();

                commands.entity(entity).add_child(collider);
            }
        }

        if attacking.0 {
            attack_timer.0.tick(time.delta());
        }

        if attack_timer.0.finished() {
            let mut count = 0;
            for i in children.unwrap().iter() {
                count += 1;
            }
            if let Some(&child) = children
                .expect("NO CHILD WHEN KILLING COLLIDER CHILD")
                .get(0)
            {
                attack_timer.0.reset();
                commands.entity(child).despawn_recursive();
            }

            attacking.0 = false;
        }
    }
}

pub fn Attack_Collider_Handler(
    mut collision_events: EventReader<CollisionEvent>,
    mut enemy_data: Query<(&Enemy, &mut EnemyStats, Entity)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, flag) => {
                if flag == &CollisionEventFlags::SENSOR {
                    let this = enemy_data.get_mut(*entity1);
                    if let Ok((enemy, mut stats, entity)) = enemy_data.get_mut(*entity1) {
                        enemy.attack(ATTACK_POWER, &mut stats);
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
