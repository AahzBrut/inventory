use bevy::prelude::*;
use crate::windows_plugin::components::UIWindow;
use crate::windows_plugin::resources::UIResources;

pub struct WindowParameters {
    pub window_type: UIWindow,
    pub position: UiRect,
    pub size: Size,
    pub border: UiRect,
    pub border_color: BackgroundColor,
    pub z_index: i32,
    pub header_height: Val,
    pub ui_resources: UIResources,
}
