use bevy::prelude::*;
use super::attack::Attack;

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub attack_power: i32,
    pub armor: i32,
}

impl Player {
    pub fn new(health: i32, attack_power: i32, armor: i32) -> Player {
        Player {
            health,
            attack_power,
            armor,
        }
    }
}

impl Attack for Player {
    fn attack(&self) {
        println!("Player attacks with power {}", self.attack_power);
    }
}

#[derive(Component)]
pub enum Direction {
    Up,
    Down
}



pub fn sprite_movement(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut sprite_position: Query<(&mut Direction, &mut Transform), With<Player>>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => {
                if keyboard_input.pressed(KeyCode::Up) {
                    transform.translation.y += 150. * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    transform.translation.y -= 150. * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::Left) {
                    transform.translation.x -= 150. * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::Right) {
                    transform.translation.x += 150. * time.delta_seconds();
                }
            },
            Direction::Down => {
                if keyboard_input.pressed(KeyCode::Up) {
                    transform.translation.y += 150. * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    transform.translation.y -= 150. * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::Left) {
                    transform.translation.x -= 150. * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::Right) {
                    transform.translation.x += 150. * time.delta_seconds();
                }
            },
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}