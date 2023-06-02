use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DraggedWindow{
    pub window: Option<Entity>,
}


#[derive(Resource, Default)]
pub struct UIResources{
    pub window_close_button: Handle<Image>
}
