#![allow(unused)]
#![allow(non_snake_case)]

mod game_core;

use bevy::prelude::*;
use bevy::window::{CompositeAlphaMode, CursorGrabMode, PresentMode, WindowResizeConstraints};

use game_core::player;
use game_core::attack;


fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 500.0,
            height: 750.0,
            position: WindowPosition::Centered,
            monitor: MonitorSelection::Primary,
            resize_constraints: WindowResizeConstraints {
                min_width: 500.0,
                min_height: 750.0,
                max_width: 500.0,
                max_height: 750.0,
            },
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
    }));

    app.run();
}