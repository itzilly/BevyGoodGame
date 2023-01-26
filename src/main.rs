#![allow(unused)]
#![allow(non_snake_case)]

pub mod game_core;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::{CompositeAlphaMode, CursorGrabMode, PresentMode, WindowResizeConstraints};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude;
use std::collections::{HashMap, HashSet};

use game_core::player;
use game_core::attack;
use crate::game_core::player::Player;

#[derive(
Copy, Clone, Eq, PartialEq, Debug, Default, Component,
)]
struct Fence;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
struct FenceBundle {
    fence: Fence,
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
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_int_cell::<FenceBundle>(1)

        // .add_plugin(WorldInspectorPlugin);  // -> Debug information panel
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup_system)
        .insert_resource(LevelSelection::Index(0))

        .add_system(player::sprite_movement)
        .add_system(spawn_fence_collision);

    app.run();
}


fn spawn_fence_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Fence>>,
    parent_query: Query<&Parent, Without<Fence>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(
    Copy, Clone, Eq, PartialEq, Debug, Default, Hash,
    )]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a
    /// wall of any size
    #[derive(
    Copy, Clone, Eq, PartialEq, Debug, Default, Hash,
    )]
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for
    // quick, easy lookup
    //
    // The key of this map will be the entity of the
    // level the wall belongs to. This has two
    // consequences in the resulting collision
    // entities: 1. it forces the walls to be
    // split along level boundaries 2. it lets us
    // easily add the collision entities as children
    // of the appropriate level entity
    let mut level_to_wall_locations: HashMap<
        Entity,
        HashSet<GridCoords>,
    > = HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer
        // entity, not the level entity To get the
        // level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) =
            parent_query.get(parent.get())
        {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_insert(HashSet::new())
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_handle)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let level = levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level
                    .level
                    .layer_instances
                    .clone()
                    .expect("Level asset should have layers")[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right
                    // edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut wall_rects: Vec<Rect> = Vec::new();
                let mut previous_rects: HashMap<Plate, Rect> = HashMap::new();

                // an extra empty row so the algorithm "terminates" the rects that touch the top
                // edge
                plate_stack.push(Vec::new());

                for (y, row) in plate_stack.iter().enumerate() {
                    let mut current_rects: HashMap<Plate, Rect> = HashMap::new();
                    for plate in row {
                        if let Some(previous_rect) = previous_rects.remove(plate) {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    top: previous_rect.top + 1,
                                    ..previous_rect
                                },
                            );
                        } else {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    bottom: y as i32,
                                    top: y as i32,
                                    left: plate.left,
                                    right: plate.right,
                                },
                            );
                        }
                    }

                    // Any plates that weren't removed above have terminated
                    wall_rects.append(&mut previous_rects.values().copied().collect());
                    previous_rects = current_rects;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn((Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ),
                                    RigidBody::Fixed,
                                    Friction{
                                        coefficient: 0.1,
                                        combine_rule:
                                        CoefficientCombineRule::Min,
                                    },
                                    Transform::from_xyz(
                                        (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                            / 2.,
                                        (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                            / 2.,
                                        0.,
                                    ),
                                    GlobalTransform::default()));
                    }
                });
            }
        });
    }
}



fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    audio: Res<Audio>
) {
    // Spawn the camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 2.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("game/characters/player/player.png"),
            transform: Transform::from_xyz(100., 0., 5.),
            ..default()
        },
        player::Direction::Up,
        Player {
            health: 10,
            attack_power: 10,
            armor: 10,
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.0),
        Collider::cuboid(64.0 / 2.0, 64.0 / 2.0),
    ));

    // Spawning the tileset "World"
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/sandbox.ldtk"),
        ..Default::default()
    });

    let startup_sound = asset_server.load("audio/sound_effects/pickup-coin.ogg");
    audio.play(startup_sound);

}



