use bevy::prelude::*;

#[derive(Component, Reflect, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[reflect(Component, Default)]
pub enum UIWindow {
    #[default]
    InventoryWindow,
    CharacterWindow,
}

#[derive(Component, Copy, Clone)]
pub struct EntityRef(pub Entity);

#[derive(Component)]
pub struct UIHeader;

#[derive(Component)]
pub struct UICloseButton;
