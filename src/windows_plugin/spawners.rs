use bevy::prelude::*;
use crate::windows_plugin::components::{EntityRef, UICloseButton, UIHeader};

use crate::windows_plugin::data::WindowParameters;

pub fn spawn_window(
    commands: &mut Commands,
    parameters: &WindowParameters,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: parameters.size,
                    position: parameters.position,
                    border: parameters.border,
                    ..default()
                },
                background_color: parameters.border_color,
                z_index: ZIndex::Global(parameters.z_index),
                ..default()
            },
            parameters.window_type,
            Name::from("Window"),
        )).with_children(|parent| {
        let root_window = EntityRef(parent.parent_entity());
        parent.spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), parameters.header_height),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                spawn_window_header(
                    parent,
                    root_window,
                    parameters);
            });
        });
    });
}

fn spawn_window_header(
    parent: &mut ChildBuilder,
    root_window: EntityRef,
    parameters: &WindowParameters,
) {
    parent.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), parameters.header_height),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn((NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), parameters.header_height),
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        },
                      Interaction::None,
                      UIHeader,
                      root_window,
        ));
        parent.spawn((ImageBundle {
            style: Style {
                flex_shrink: 0.0,
                flex_grow: 0.0,
                size: Size::new(parameters.header_height, parameters.header_height),
                ..default()
            },
            image: parameters.ui_resources.window_close_button.clone().into(),
            ..default()
        },
                      Interaction::None,
                      UICloseButton,
                      root_window
        ));
    });
}
