// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use std::collections::HashMap;

use bevy::asset::Handle;
use bevy::prelude::{Component, Deref, DerefMut, Entity, Event, Image, Resource, Timer};
use bevy::sprite::TextureAtlasLayout;

#[cfg(feature = "asset_loader")]
use bevy_2dviewangle_common::ActorsTexturesLoader;

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
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

#[derive(Default, Deref, DerefMut)]
pub struct ViewTextures(HashMap<Angle, ViewSprite>);

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

impl ViewTextures {
    pub fn from(items: Vec<(Angle, ViewSprite)>) -> Self {
        let mut map = HashMap::new();
        for (key, value) in items {
            map.insert(key, value);
        }
        Self(map)
    }
}

impl ActorsTextures {
    // #[cfg(feature = "asset_loader")]
    pub fn load_asset_loader<T: ActorsTexturesLoader>(&mut self, loader: &T) {
        let fields = loader.get_all();
        for field in fields {
            let field_angle = match field.angle.as_str() {
                "front" => Angle::Front,
                "back" => Angle::Back,
                "left" => Angle::Left,
                "right" => Angle::Right,
                "front_left" => Angle::FrontLeft,
                "front_right" => Angle::FrontRight,
                "back_left" => Angle::BackLeft,
                "back_right" => Angle::BackRight,
                _ => Angle::Front,
            };
            let mut actor;
            if let Some(mut _actor) = self.get_mut(&field.actor) {
                actor = _actor;
            } else {
                self.insert(field.actor, HashMap::default());
                actor = self.get_mut(&field.actor).unwrap();
            }

            let mut action;
            if let Some(mut _action) = actor.get_mut(&field.action) {
                action = _action;
            } else {
                actor.insert(field.action, ViewTextures::default());
                action = actor.get_mut(&field.action).unwrap();
            }

            let mut sprite;
            if let Some(mut _sprite) = action.get_mut(&field_angle) {
                sprite = _sprite;
            } else {
                action.insert(field_angle, ViewSprite::default());
                sprite = action.get_mut(&field_angle).unwrap();
            }

            if let Some(image) = field.image {
                sprite.image = image.clone();
            }
            if let Some(atlas_layout) = field.atlas_layout {
                sprite.layout = atlas_layout;
            }
        }
    }
}
