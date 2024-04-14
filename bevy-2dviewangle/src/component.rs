// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use std::collections::HashMap;

use bevy::asset::Handle;
use bevy::prelude::{Component, Deref, DerefMut, Entity, Event, Image, Resource, Timer};
use bevy::sprite::TextureAtlasLayout;
pub use bevy_2dviewangle_macro::ActorsTexturesCollection;

#[derive(Default)]
pub struct FieldInfo<'a> {
    pub actor: Option<u64>,
    pub action: Option<u16>,
    pub angle: Option<String>,
    pub image: Option<&'a Handle<Image>>,
    pub atlas_layout: Option<&'a Handle<TextureAtlasLayout>>,
}

pub trait ActorsTexturesCollection {
    fn get_all(&self) -> Vec<FieldInfo>;
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Angle {
    Any,
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

#[derive(Default, Clone)]
pub struct ViewSprite {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
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
    pub fn load_asset_loader<T: ActorsTexturesCollection>(&mut self, loader: &T) {
        let fields = loader.get_all();
        let mut actor_id = 0;
        let mut action_id = 0;
        for field in fields {
            actor_id = field.actor.unwrap_or(actor_id);
            action_id = field.action.unwrap_or(action_id);
            let field_angle = match field.angle.unwrap_or_default().as_str() {
                "front" => Angle::Front,
                "back" => Angle::Back,
                "left" => Angle::Left,
                "right" => Angle::Right,
                "front_left" => Angle::FrontLeft,
                "front_right" => Angle::FrontRight,
                "back_left" => Angle::BackLeft,
                "back_right" => Angle::BackRight,
                _ => Angle::Any,
            };
            let actor;
            if let Some(_actor) = self.get_mut(&actor_id) {
                actor = _actor;
            } else {
                self.insert(actor_id, HashMap::default());
                actor = self.get_mut(&actor_id).unwrap();
            }

            let action;
            if let Some(_action) = actor.get_mut(&action_id) {
                action = _action;
            } else {
                actor.insert(action_id, ViewTextures::default());
                action = actor.get_mut(&action_id).unwrap();
            }

            let any = action.get(&Angle::Any).cloned();
            let sprite;
            if let Some(_sprite) = action.get_mut(&field_angle) {
                sprite = _sprite;
            } else {
                action.insert(field_angle, ViewSprite::default());
                sprite = action.get_mut(&field_angle).unwrap();
            }

            if let Some(image) = field.image {
                sprite.image = Some(image.clone());
            } else if any.is_some() {
                sprite.image = any.as_ref().unwrap().image.clone();
            }

            if let Some(atlas_layout) = field.atlas_layout {
                sprite.layout = Some(atlas_layout.clone());
            } else if any.is_some() {
                sprite.layout = any.unwrap().layout.clone();
            }

            if field_angle == Angle::Any {
                let any = sprite.clone();
                for s in action.values_mut() {
                    if s.image.is_none() {
                        s.image = any.image.clone();
                    }
                    if s.layout.is_none() {
                        s.layout = any.layout.clone();
                    }
                }
            }
        }
    }
}
