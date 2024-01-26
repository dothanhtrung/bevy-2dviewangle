// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use bevy::asset::Handle;
use bevy::prelude::{Component, Deref, DerefMut, Entity, Event, Resource};
use bevy::sprite::TextureAtlas;
use std::collections::HashMap;

#[derive(Default, Clone, Copy, Eq, PartialEq)]
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

#[derive(Component, Default, Deref, DerefMut)]
pub struct Direction(ViewDirection);

#[derive(Component, Default)]
pub struct Dynamic2DView {
    pub direction: ViewDirection,
    pub action: u16,
    pub actor: u64,
    pub flipped: bool,
}

#[derive(Component, Default)]
pub struct Static2DView {
    pub actor: u64,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Animation2D(HashMap<u64, HashMap<u16, TextureViewCollections>>);

#[derive(Event)]
pub struct ViewChanged {
    pub entity: Entity,
}
