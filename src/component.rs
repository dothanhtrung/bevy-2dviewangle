use bevy::asset::Handle;
use bevy::prelude::{Component, Deref, DerefMut, Entity, Event};
use bevy::sprite::TextureAtlas;
use std::collections::HashMap;

#[derive(Default, Clone, Copy)]
pub enum ViewDirection {
    #[default]
    Front,
    Back,
    Left,
    Right,
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight,
}

#[derive(Default)]
pub struct TextureViewCollections {
    pub front: Option<Handle<TextureAtlas>>,
    pub back: Option<Handle<TextureAtlas>>,
    pub left: Option<Handle<TextureAtlas>>,
    pub right: Option<Handle<TextureAtlas>>,
    pub front_left: Option<Handle<TextureAtlas>>,
    pub front_right: Option<Handle<TextureAtlas>>,
    pub back_left: Option<Handle<TextureAtlas>>,
    pub back_right: Option<Handle<TextureAtlas>>,
}

// pub trait Animation2D {
//     fn front(&self) -> Some(Handle<TextureAtlas>);
//     fn back(&self) -> Some(Handle<TextureAtlas>);
//     fn left(&self) -> Some(Handle<TextureAtlas>);
//     fn right(&self) -> Some(Handle<TextureAtlas>);
//     fn front_left(&self) -> Some(Handle<TextureAtlas>);
//     fn front_right(&self) -> Some(Handle<TextureAtlas>);
//     fn back_right(&self) -> Some(Handle<TextureAtlas>);
//     fn back_left(&self) -> Some(Handle<TextureAtlas>);
// }

#[derive(Component, Default, Deref, DerefMut)]
pub struct Direction(ViewDirection);

#[derive(Component)]
pub struct ActionDirection {
    pub direction: ViewDirection,
    pub action: u8,
    pub animation: HashMap<u8, TextureViewCollections>,
}

#[derive(Event)]
pub struct DirectionChanged {
    pub direction: ViewDirection,
    pub entity: Entity,
}

#[derive(Event)]
pub struct ActionChanged {
    pub action: u8,
    pub entity: Entity,
}
