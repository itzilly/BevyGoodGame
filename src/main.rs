#![allow(unused)]
#![allow(non_snake_case)]

mod game_core;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::{CompositeAlphaMode, CursorGrabMode, PresentMode, WindowResizeConstraints};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ecs_ldtk::prelude::*;

use game_core::player;
use game_core::attack;

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

#[derive(Component)]
enum Direction {
    Up,
    Down
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 1024.0,
            height: 576.0,
            position: WindowPosition::Centered,
            monitor: MonitorSelection::Primary,
            resize_constraints: WindowResizeConstraints::default(),
            scale_factor_override: None,
            title: "Bevy Good Game".to_string(),
            present_mode: PresentMode::AutoVsync,
            resizable: false,
            decorations: true,
            cursor_visible: true,
            cursor_grab_mode: CursorGrabMode::None,
            mode: WindowMode::Windowed,
            transparent: false,
            canvas: None,
            fit_canvas_to_parent: false,
            alpha_mode: CompositeAlphaMode::Auto,
        },
        add_primary_window: true,
        exit_on_all_closed: true,
        close_when_requested: true,
    }))

        // .add_plugin(WorldInspectorPlugin);  // -> Debug information panel
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup_system)
        .insert_resource(LevelSelection::Index(0))
        .add_system(sprite_movement);

    app.run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    audio: Res<Audio>
) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("game/characters/player/player.png"),
            transform: Transform::from_xyz(100., 0., 1.),
            ..default()
        },
        Direction::Up,
    ));

    // Spawning the tileset "World"
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/sandbox.ldtk"),
        ..Default::default()
    });

    let startup_sound = asset_server.load("audio/sound_effects/pickup-coin.ogg");
    audio.play(startup_sound);

}

fn sprite_movement(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
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