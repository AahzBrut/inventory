use bevy::prelude::*;

#[derive(Component, Reflect, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UIWindow {
    InventoryWindow,
    CharacterWindow,
}

#[derive(Component)]
pub struct UIHeader;
