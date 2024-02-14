// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use bevy::asset::Handle;
use bevy::prelude::{Component, Deref, DerefMut, Entity, Event, Resource, Timer};
use bevy::sprite::TextureAtlas;
use std::collections::HashMap;

#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub enum Angle {
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
pub struct ViewTextures {
    pub front: Option<Handle<TextureAtlas>>,
    pub back: Option<Handle<TextureAtlas>>,
    pub left: Option<Handle<TextureAtlas>>,
    pub right: Option<Handle<TextureAtlas>>,
    pub front_left: Option<Handle<TextureAtlas>>,
    pub front_right: Option<Handle<TextureAtlas>>,
    pub back_left: Option<Handle<TextureAtlas>>,
    pub back_right: Option<Handle<TextureAtlas>>,
}

#[derive(Component, Default)]
pub struct DynamicActor {
    pub angle: Angle,
    pub action: u16,
    pub actor: u64,
    pub flipped: bool,
    pub animation_timer: Option<Timer>,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ActorsTextures(HashMap<u64, HashMap<u16, ViewTextures>>);

#[derive(Event)]
pub struct ViewChanged {
    pub entity: Entity,
}
