#![allow(unused)]
#![allow(non_snake_case)]

pub mod game_core;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::{CompositeAlphaMode, CursorGrabMode, PresentMode, WindowResizeConstraints};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use crate::game_core::player::Player;
use game_core::attack;
use game_core::camera;
use game_core::player;
use game_core::world;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
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
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(LdtkPlugin)
    .insert_resource(LevelSelection::Index(0))
    .register_ldtk_int_cell::<world::FenceBundle>(1)
    .register_ldtk_entity::<player::PlayerBundle>("Player")
    // .add_plugin(WorldInspectorPlugin);  // -> Debug information panel
    .add_startup_system(setup_system)
    .insert_resource(LevelSelection::Index(0))
    .add_system(player::player_movement_system)
    .add_system(camera::follow_player_system)
    .add_system(world::spawn_fence_collision);

    app.run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    audio: Res<Audio>,
) {
    // Spawn the camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.4,
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawning the tileset "World"
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/sandbox.ldtk"),
        ..Default::default()
    });

    let startup_sound = asset_server.load("audio/sound_effects/pickup-coin.ogg");
    audio.play(startup_sound);
}
