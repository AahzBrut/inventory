#![allow(
clippy::module_name_repetitions,
clippy::missing_panics_doc,
clippy::needless_pass_by_value,
clippy::type_complexity
)]

use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::windows_plugin::components::UIWindow;
use crate::windows_plugin::data::WindowParameters;
use crate::windows_plugin::resources::UIResources;
use crate::windows_plugin::spawners::spawn_window;
use crate::windows_plugin::WindowsPlugin;

mod windows_plugin;

fn main() {
    App::new()
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
        .add_plugin(WindowsPlugin)
        .add_startup_system(load_resources.in_base_set(StartupSet::PreStartup))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_inventory_window)
        .add_startup_system(spawn_character_window)
        .run();
}

pub fn spawn_inventory_window(
    mut commands: Commands,
    ui_resources: Res<UIResources>,
) {
    spawn_window(
        &mut commands,
        &WindowParameters {
            z_index: 1,
            window_type: UIWindow::InventoryWindow,
            position: UiRect::new(Val::Px(100.0), Val::Undefined, Val::Px(100.0), Val::Undefined),
            ui_resources: ui_resources.clone(),
            size: Size::all(Val::Px(300.0)),
            header_height: Val::Px(32.0),
            border_color: Color::RED.into(),
            border: UiRect::all(Val::Px(2.0)),
        });
}

pub fn spawn_character_window(
    mut commands: Commands,
    ui_resources: Res<UIResources>,
) {
    spawn_window(
        &mut commands,
        &WindowParameters {
            z_index: 1,
            window_type: UIWindow::CharacterWindow,
            position: UiRect::new(Val::Px(500.0), Val::Undefined, Val::Px(100.0), Val::Undefined),
            ui_resources: ui_resources.clone(),
            size: Size::all(Val::Px(300.0)),
            header_height: Val::Px(32.0),
            border_color: Color::RED.into(),
            border: UiRect::all(Val::Px(2.0)),
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

pub fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(
        UIResources {
            window_close_button: asset_server.load("ui/buttons/CloseButton.png"),
        }
    );
}
