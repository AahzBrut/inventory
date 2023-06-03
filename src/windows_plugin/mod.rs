use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::windows_plugin::components::UIWindow;
use crate::windows_plugin::resources::DraggedWindow;
use crate::windows_plugin::systems::drag_window::drag_window;
use crate::windows_plugin::systems::start_drag_window::start_drag_window;

pub mod components;
pub mod resources;
pub mod spawners;
pub mod systems;
pub mod data;


pub struct WindowsPlugin;

impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UIWindow>()
            .register_type::<ZIndex>()
            .insert_resource(DraggedWindow { window: None })
            .add_system(start_drag_window)
            .add_system(drag_window)
        ;
    }
}
