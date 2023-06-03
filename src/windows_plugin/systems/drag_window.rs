use bevy::prelude::*;
use bevy::math::Vec2;
use bevy::window::PrimaryWindow;
use crate::windows_plugin::resources::DraggedWindow;

pub fn drag_window(
    mut last_position: Local<Vec2>,
    mut window_style_query: Query<&mut Style>,
    dragged_window: Res<DraggedWindow>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    if let Some(cursor_position) = window.cursor_position() {
        if let Some(window_entity) = dragged_window.window {
            let mut delta = cursor_position - *last_position;
            delta.y = -delta.y;

            if let Ok(mut style) = window_style_query.get_component_mut::<Style>(window_entity){
                style.position.left.try_add_assign(Val::Px(delta.x)).unwrap();
                style.position.top.try_add_assign(Val::Px(delta.y)).unwrap();
            }
        }
        *last_position = cursor_position;
    }
}
