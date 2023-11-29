use bevy::prelude::*;

pub enum MenuItemType {
    NewGame,
    LevelEditor,
    Exit,
}

#[derive(Component, Default)]
pub struct MenuBlock;

#[derive(Component)]
pub struct MenuItem {
    pub item_type: MenuItemType,
}

impl MenuItem {
    pub fn new(item_type: MenuItemType) -> MenuItem {
        MenuItem { item_type }
    }
}