// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use std::collections::HashMap;

use bevy::asset::Handle;
use bevy::prelude::{Component, Deref, DerefMut, Entity, Event, Image, Resource, Timer};
use bevy::sprite::TextureAtlasLayout;
pub use bevy_2dviewangle_macro::ActorsTexturesCollection;

/// The trait to use in derive macro. You won't need to implement this trait.
///
/// Example:
/// ```rust
/// use bevy::prelude::*;
/// use bevy_2dviewangle::ActorsTexturesCollection;
///
/// #[derive(ActorsTexturesCollection)]
/// pub struct MyAssets {
///     #[textureview(actor = "frog", action = "idle", angle = "front")]
///     pub idle_front: Handle<Image>,
///
///     // If not specify actor/action, the previous value will be used
///     #[textureview(angle = "back")]
///     pub idle_back: Handle<Image>,
///
///     // If the angle "right" is not defined, it will be flipped (2d) or rotate (3d) base on the angle "left" image
///     #[textureview(angle = "left")]
///     pub idle_left: Handle<Image>,
///
///     // If angle is any, other angle which has not been defined will use this value
///     #[textureview(angle = "any")]
///     pub layout: Handle<TextureAtlasLayout>,
/// }
/// ```
///
/// Two enums will be generated base on declared actor and action:
/// ```rust
/// #[derive(Default, Eq, PartialEq)]
/// #[repr(u64)]
/// pub enum ActorMyAssets {
///     #[default]
///     Any,
///     Frog,
/// }
///
/// #[derive(Default, Eq, PartialEq)]
/// #[repr(u16)]
/// pub enum ActionMyAssets {
///     #[default]
///     Any,
///     Idle,
/// }
/// ```
pub trait ActorsTexturesCollection {
    fn get_all(
        &self,
    ) -> Vec<(
        Option<u64>,
        Option<u16>,
        Option<Angle>,
        Option<&Handle<Image>>,
        Option<&Handle<TextureAtlasLayout>>,
    )>;
}

/// All supported angles.
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

/// Sprite sheet for one angle, store image and atlas layout
#[derive(Default, Clone)]
pub struct ViewSprite {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

/// Map of Angle and its ViewSprite
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

/// The resource that stores every spritesheets. Organized by actor id (u64) and action id (u16)
#[derive(Resource, Deref, DerefMut, Default)]
pub struct ActorsTextures(HashMap<u64, HashMap<u16, ViewTextures>>);

/// Event to send when want to change the spritesheet.
///
/// Example:
/// ```rust
/// use bevy::prelude::*;
/// use bevy_2dviewangle::{Angle, DynamicActor, ViewChanged};
///
/// pub fn input(
///     mut actors: Query<(&mut DynamicActor, Entity)>,
///     mut action_event: EventWriter<ViewChanged>,
/// ) {
///     for (mut act, e) in actors.iter_mut() {
///             act.action = Action::Idle as u16;
///             act.angle = Angle::Left;
///             // Send event to change to sprite sheet to another view
///             action_event.send(ViewChanged { entity: e });
///         }
///     }
/// }
/// ```
#[derive(Event)]
pub struct ViewChanged {
    pub entity: Entity,
}

impl ViewTextures {
    /// Store spritesheets from list of Angle and ViewSprite in case you don't want to use derive `ActorsTexturesCollection`.
    pub fn from(items: Vec<(Angle, ViewSprite)>) -> Self {
        let mut map = HashMap::new();
        for (key, value) in items {
            map.insert(key, value);
        }
        Self(map)
    }
}

impl ActorsTextures {
    /// Store spiresheets from an instance of struct that uses derive `ActorsTexturesCollection`.
    ///
    /// Example:
    /// ```rust
    /// use bevy::prelude::*;
    /// use bevy_2dviewangle::{ActorsTextures, ActorsTexturesCollection};
    ///
    /// #[derive(ActorsTexturesCollection)]
    /// pub struct MyAssets {
    ///     #[textureview(actor = "frog", action = "idle", angle = "front")]
    ///     pub idle_front: Handle<Image>,
    ///
    ///     #[textureview(angle = "back")]
    ///     pub idle_back: Handle<Image>,
    ///
    ///     #[textureview(angle = "left")]
    ///     pub idle_left: Handle<Image>,
    ///
    ///     #[textureview(angle = "any")]
    ///     pub layout: Handle<TextureAtlasLayout>,
    /// }
    ///
    /// fn setup(
    ///     mut commands: Commands,
    ///     asset_server: Res<AssetServer>,
    ///     mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ///     mut animation2d: ResMut<ActorsTextures>,
    /// ) {
    ///     let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 1, 3, None, None);
    ///     let my_assets = MyAssets {
    ///         idle_front: asset_server.load("frog_idle_front.png"),
    ///         idle_back: asset_server.load("frog_idle_back.png"),
    ///         idle_left: asset_server.load("frog_idle_left.png"),
    ///         layout: texture_atlases.add(layout),
    ///     };
    ///
    ///     // Load into collection
    ///     animation2d.load_asset_loader(&my_assets);
    /// }
    /// ```
    pub fn load_asset_loader<T: ActorsTexturesCollection>(&mut self, loader: &T) {
        let mut actor_id = 0;
        let mut action_id = 0;
        for (actor, action, angle, image, atlas_layout) in loader.get_all() {
            actor_id = actor.unwrap_or(actor_id);
            action_id = action.unwrap_or(action_id);
            let field_angle = angle.unwrap_or_default();
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

            if let Some(image_handle) = image {
                sprite.image = Some(image_handle.clone());
            } else if any.is_some() {
                sprite.image = any.as_ref().unwrap().image.clone();
            }

            if let Some(atlas_layout_handle) = atlas_layout {
                sprite.layout = Some(atlas_layout_handle.clone());
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
