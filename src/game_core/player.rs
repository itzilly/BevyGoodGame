use bevy::prelude::*;
use bevy_inspector_egui::egui::Event::Key;
use super::attack::Attack;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub attack_power: i32,
    pub armor: i32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub max_speed: f32,
}

impl Player {
    pub fn new(health: i32, attack_power: i32, armor: i32, acceleration: f32, deceleration: f32, max_speed: f32) -> Player {
        Player {
            health,
            attack_power,
            armor,
            acceleration,
            deceleration,
            max_speed,
        }
    }
}

impl Attack for Player {
    fn attack(&self) {
        println!("Player attacks with power {}", self.attack_power);
    }
}

pub fn player_movement_system(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut player: Query<(&mut Velocity, &Player, &Transform)>) {
    // Get player information
    if let Ok((mut player_velocity, player, transform)) = player.get_single_mut() {

        // If any movement keys pressed
        if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W, KeyCode::Left, KeyCode::A, KeyCode::Down, KeyCode::S, KeyCode::Right, KeyCode::D]) {

            // Acceleration is the rate at wich the speed increases
            let mut acceleration = Vec2::ZERO;

            // set the acceleration based on inputs
            if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                acceleration += Vec2::new(0.0, 1.0) * player.acceleration;
            }
            if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                acceleration += Vec2::new(1.0, 0.0) * player.acceleration;
            }
            if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                acceleration += Vec2::new(0.0, -1.0) * player.acceleration;
            }
            if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                acceleration += Vec2::new(-1.0, 0.0) * player.acceleration;
            }

            // Only if the acceleration is zero (ONLY IN CASES WHERE ALL ARE PRESSED)
            if acceleration.length_squared() == 0.0 {
                return;
            }

            // making sure the diagonals arent faster than they should be
            acceleration = acceleration.normalize();

            let velocity_dot_acceleration = player_velocity.linvel.normalize().dot(acceleration.normalize());

            // When changing directions, accelerate more to make the turn faster
            if velocity_dot_acceleration < 0.9 {
                acceleration = acceleration * 4.0;
            }
            else if velocity_dot_acceleration > 0.9 && velocity_dot_acceleration < 0.98 {
                acceleration = (acceleration.normalize() - (player_velocity.linvel.normalize() * 0.8)).normalize()
            }




            // apply the velocity
            player_velocity.linvel += acceleration * player.acceleration * time.delta_seconds();

            // Cap speed if beyond max
            if player_velocity.linvel.length() > player.max_speed {
                player_velocity.linvel = player_velocity.linvel.normalize() * player.max_speed;
            }
            // If no input AND if after this loop the velocity will be greater than zero
        } else if player_velocity.linvel.length() - (player.deceleration * time.delta_seconds()) > 0.0 {
            // Decelerate
            player_velocity.linvel = player_velocity.linvel - player_velocity.linvel.normalize() * player.deceleration * time.delta_seconds();
            return;
        } else { // Stop the player
            player_velocity.linvel = Vec2::ZERO;
            return;
        }


    }


}