use bevy::prelude::Resource;
use crate::components::UIWindow;

#[derive(Resource)]
pub struct DraggedWindow{
    pub window: Option<UIWindow>,
}
