#![allow(
clippy::missing_panics_doc,
clippy::needless_pass_by_value,
clippy::type_complexity
)]

use std::collections::HashMap;
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::components::UIWindow;
use crate::resources::DraggedWindow;

mod components;
mod resources;

fn main() {
    App::new()
        .register_type::<UIWindow>()
        .register_type::<ZIndex>()
        .insert_resource(DraggedWindow { window: None })
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::from((800.0, 600.0)),
                    title: "Inventory".into(),
                    resizable: true,
                    present_mode: PresentMode::AutoVsync,
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_inventory_window)
        .add_startup_system(spawn_character_window)
        .add_system(start_drag_window)
        .add_system(drag_window)
        .run();
}

pub fn drag_window(
    mut last_position: Local<Vec2>,
    mut dragged_window_query: Query<(&mut Style, &UIWindow), Without<Interaction>>,
    dragged_window: Res<DraggedWindow>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    if let Some(cursor_position) = window.cursor_position() {
        if dragged_window.window.is_none() {
            *last_position = cursor_position;
            return;
        }
        let mut delta = cursor_position - *last_position;
        delta.y = -delta.y;
        dragged_window_query.for_each_mut(|(mut style, &ui_window)| {
            if ui_window == dragged_window.window.unwrap() {
                style.position.left.try_add_assign(Val::Px(delta.x)).unwrap();
                style.position.top.try_add_assign(Val::Px(delta.y)).unwrap();
            }
        });
        *last_position = cursor_position;
    }
}

pub fn start_drag_window(
    mut dragged_window: ResMut<DraggedWindow>,
    mut windows_query: Query<(&mut ZIndex, &UIWindow), Without<Interaction>>,
    ui_window_query: Query<(&Interaction, &UIWindow), Changed<Interaction>>,
) {
    if let Ok((interaction, &window)) = ui_window_query.get_single() {
        match interaction {
            Interaction::Clicked => {
                dragged_window.window = Some(window);
                update_z_indexes(&mut windows_query, window);
            }
            Interaction::Hovered | Interaction::None => {
                if let Some(current_window) = dragged_window.window {
                    if current_window != window { return; }
                }
                dragged_window.window = None;
            }
        }
    }
}

fn update_z_indexes(windows_query: &mut Query<(&mut ZIndex, &UIWindow), Without<Interaction>>, window: UIWindow) {
    let win_count = i32::try_from(windows_query.iter().count()).unwrap();
    let win_z_map = get_sorted_windows_map(windows_query, window);
    windows_query.for_each_mut(|(mut z_index, &window_type)| {
        if window_type == window {
            *z_index = ZIndex::Global(win_count);
        } else {
            *z_index = ZIndex::Global(*win_z_map.get(&window_type).unwrap());
        }
    });
}

fn get_sorted_windows_map(
    windows_query: &mut Query<(&mut ZIndex, &UIWindow), Without<Interaction>>,
    window: UIWindow
) -> HashMap<UIWindow, i32> {
    let mut windows_z = windows_query.iter()
        .filter(|(&z, &w)| { w != window && matches!(z, ZIndex::Global(_))})
        .map(|(z, w)| {
            if let ZIndex::Global(z) = z {
                (*w, *z)
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
    windows_z.iter().map(|(w, z)| { (*w, *z) }).collect::<HashMap<_, _>>()
}

pub fn spawn_inventory_window(
    mut commands: Commands,
) {
    commands.spawn((NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::all(Val::Px(300.0)),
            position: UiRect::new(Val::Px(100.0), Val::Auto, Val::Px(100.0), Val::Auto),
            ..default()
        },
        background_color: Color::GREEN.into(),
        z_index: ZIndex::Global(1),
        ..default()
    },
                    UIWindow::InventoryWindow,
                    Name::from("Inventory window"),
    )).with_children(|parent| {
        spawn_window_header(parent, 300.0, 24.0, UIWindow::InventoryWindow);
    });
}

pub fn spawn_character_window(
    mut commands: Commands,
) {
    commands.spawn((NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::all(Val::Px(300.0)),
            position: UiRect::new(Val::Px(500.0), Val::Auto, Val::Px(100.0), Val::Auto),
            ..default()
        },
        z_index: ZIndex::Global(2),
        background_color: Color::BLUE.into(),
        ..default()
    },
                    UIWindow::CharacterWindow,
                    Name::from("Character window"),
    )).with_children(|parent| {
        spawn_window_header(parent, 300.0, 24.0, UIWindow::CharacterWindow);
    });
}

fn spawn_window_header(
    parent: &mut ChildBuilder,
    header_width: f32,
    header_height: f32,
    window_type: UIWindow,
) {
    parent.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(header_width), Val::Px(header_height)),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn((NodeBundle {
            style: Style {
                size: Size::new(Val::Px(header_width - header_height), Val::Px(header_height)),
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        },
                      Interaction::None,
                      window_type,
        ));
        parent.spawn((NodeBundle {
            style: Style {
                size: Size::new(Val::Px(header_height), Val::Px(header_height)),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        },
                      Interaction::None,
                      window_type
        ));
    });
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() * 0.5, window.height() * 0.5, 0.0),
        ..default()
    });
}
