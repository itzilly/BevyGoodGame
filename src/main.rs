#![allow(unused)]
#![allow(non_snake_case)]

mod game_core;

mod core;

use bevy::prelude::*;



fn main() {
    let mut app = App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin::default()))
        .add_system(core::systems::collision_system)
        .run();
}
