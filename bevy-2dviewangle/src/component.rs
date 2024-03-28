// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use std::collections::HashMap;

use bevy::asset::Handle;
use bevy::prelude::{Component, Deref, DerefMut, Entity, Event, Image, Resource, Timer};
use bevy::sprite::TextureAtlasLayout;

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
pub struct ViewSprite {
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}

#[derive(Default)]
pub struct ViewTextures {
    pub front: Option<ViewSprite>,
    pub back: Option<ViewSprite>,
    pub left: Option<ViewSprite>,
    pub right: Option<ViewSprite>,
    pub front_left: Option<ViewSprite>,
    pub front_right: Option<ViewSprite>,
    pub back_left: Option<ViewSprite>,
    pub back_right: Option<ViewSprite>,
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

impl ActorsTextures {
    pub fn load_asset_loader(&mut self) {

    }
}