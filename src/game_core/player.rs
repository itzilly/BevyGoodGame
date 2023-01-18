use super::attack::Attack;

pub struct Player {
    health: i32,
    attack_power: i32,
    armor: i32,
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