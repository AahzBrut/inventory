use bevy::prelude::*;
use std::collections::HashMap;
use crate::windows_plugin::components::{EntityRef, UIHeader, UIWindow};
use crate::windows_plugin::resources::DraggedWindow;

pub fn start_drag_window(
    mut dragged_window: ResMut<DraggedWindow>,
    mut windows_query: Query<(Entity, &mut ZIndex), (Without<Interaction>, With<UIWindow>)>,
    ui_window_query: Query<(&Interaction, &EntityRef), (Changed<Interaction>, With<UIHeader>)>,
) {
    if let Ok((interaction, &EntityRef(root_window))) = ui_window_query.get_single() {
        match interaction {
            Interaction::Clicked => {
                dragged_window.window = Some(root_window);
                update_z_indexes(&mut windows_query, root_window);
            }
            Interaction::Hovered | Interaction::None => {
                if let Some(current_window) = dragged_window.window {
                    if current_window != root_window { return; }
                }
                dragged_window.window = None;
            }
        }
    }
}

fn update_z_indexes(
    windows_query: &mut Query<(Entity, &mut ZIndex), (Without<Interaction>, With<UIWindow>)>,
    root_window: Entity) {
    let win_count = i32::try_from(windows_query.iter().count()).unwrap();
    let win_z_map = get_sorted_windows_map(windows_query, root_window);
    windows_query.for_each_mut(|(entity, mut z_index)| {
        if root_window == entity {
            *z_index = ZIndex::Global(win_count);
        } else {
            *z_index = ZIndex::Global(*win_z_map.get(&entity).unwrap());
        }
    });
}

fn get_sorted_windows_map(
    windows_query: &mut Query<(Entity, &mut ZIndex), (Without<Interaction>, With<UIWindow>)>,
    window: Entity,
) -> HashMap<Entity, i32> {
    let mut windows_z = windows_query.iter()
        .filter(|(e, &z)| { *e != window && matches!(z, ZIndex::Global(_)) })
        .map(|(e, z)| {
            if let ZIndex::Global(z) = z {
                (e, *z)
            } else {
                panic!("ZIndex::Local found.")
            }
        }).collect::<Vec<_>>();
    windows_z.sort_by(|o1, o2| {
        o1.1.cmp(&o2.1)
    });
    let mut new_index = 1i32;
    for (_, z) in &mut windows_z {
        *z = new_index;
        new_index += 1;
    }
    windows_z.iter().map(|(e, z)| { (*e, *z) }).collect::<HashMap<_, _>>()
}
