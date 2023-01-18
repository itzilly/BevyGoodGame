#![allow(unused)]
#![allow(non_snake_case)]


use bevy::prelude::*;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy Good Game".to_string(),
                width: 400.0,
                height: 700.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .run();
}