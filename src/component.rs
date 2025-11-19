// Copyright 2024,2025 Trung Do <dothanhtrung@pm.me>

use std::collections::HashMap;

use bevy::asset::Handle;
use bevy::prelude::{
    Component,
    Deref,
    DerefMut,
    Entity,
    EntityEvent,
    Image,
    Message,
    Reflect,
    ReflectComponent,
    Resource,
    TextureAtlasLayout,
    Timer,
};
pub use bevy_2dviewangle_macro::View2dCollection;
use xxhash_rust::xxh3::xxh3_64;

/// The trait to use in derive macro. You won't need to implement this trait.
///
/// Example:
/// ```rust
/// use bevy::prelude::*;
/// use bevy_2dviewangle::View2dCollection;
///
/// #[derive(View2dCollection)]
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
/// #[derive(Eq, PartialEq, Clone)]
/// pub enum ActorMyAssets {
///     Frog,
/// }
///
/// #[derive(Eq, PartialEq, Clone)]
/// pub enum ActionMyAssets {
///     Idle,
/// }
/// ```
pub trait View2dCollection {
    fn get_all(
        &self,
    ) -> Vec<(
        Option<u64>,
        Option<u64>,
        Option<Angle>,
        Option<&Handle<Image>>,
        Option<&Handle<TextureAtlasLayout>>,
    )>;
}

/// All supported angles.
#[derive(Reflect, Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
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
pub struct SpriteSheet {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

/// Map of Angle and its SpriteSheet  
#[derive(Default, Deref, DerefMut)]
pub struct AngleSpriteSheets(HashMap<Angle, SpriteSheet>);

#[derive(Reflect)]
pub enum Notification {
    LastFrame,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct View2dActor {
    pub angle: Angle,
    pub action: u64,
    /// Next action when the last frame of the current action is done
    pub next_action: Vec<u64>,
    pub actor: u64,
    pub flipped: bool,
    pub animation_timer: Option<Timer>,
    pub notify: Vec<Notification>,
}

/// The resource that stores every spritesheets. Organized by actor id and action id.
#[derive(Resource, Deref, DerefMut, Default)]
pub struct ActorSpriteSheets(HashMap<u64, HashMap<u64, AngleSpriteSheets>>);

/// Notify the view is changed.
///
/// Example:
/// ```rust
/// use bevy::prelude::*;
/// use bevy_2dviewangle::{Angle, View2dActor, ViewChanged};
///
/// pub fn input(
///     mut actors: Query<(&mut View2dActor, Entity)>,
///     mut action_event: MessageWriter<ViewChanged>,
/// ) {
///     for (mut act, e) in actors.iter_mut() {
///             act.action = ActionMyAssets::Idle.into();
///             act.angle = Angle::Left;
///             // Send event to change to sprite sheet to another view
///             action_event.write(ViewChanged { entity: e });
///         }
///     }
/// }
/// ```
#[derive(Message)]
pub struct ViewChanged {
    pub entity: Entity,
}

/// Sent when animation went to the last frame
#[derive(EntityEvent)]
pub struct LastFrame {
    pub entity: Entity,
}

impl AngleSpriteSheets {
    /// Store spritesheets from list of Angle and SpriteSheet in case you don't want to use derive `View2dCollection`.
    pub fn from(items: Vec<(Angle, SpriteSheet)>) -> Self {
        let mut map = HashMap::new();
        for (key, value) in items {
            map.insert(key, value);
        }
        Self(map)
    }
}

impl ActorSpriteSheets {
    /// Store spiresheets from an instance of struct that uses derive `View2dCollection`.
    ///
    /// Example:
    /// ```rust
    /// use bevy::prelude::*;
    /// use bevy_2dviewangle::{ActorSpriteSheets, View2dCollection};
    ///
    /// #[derive(View2dCollection)]
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
    ///     mut animation2d: ResMut<ActorSpriteSheets>,
    /// ) {
    ///     let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 1, 3, None, None);
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
    pub fn load_asset_loader<T: View2dCollection>(&mut self, loader: &T) {
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
                actor.insert(action_id, AngleSpriteSheets::default());
                action = actor.get_mut(&action_id).unwrap();
            }

            let any = action.get(&Angle::Any).cloned();
            let sprite;
            if let Some(_sprite) = action.get_mut(&field_angle) {
                sprite = _sprite;
            } else {
                action.insert(field_angle, SpriteSheet::default());
                sprite = action.get_mut(&field_angle).unwrap();
            }

            if let Some(image_handle) = image {
                sprite.image = Some(image_handle.clone());
            } else if let Some(any) = any.as_ref() {
                sprite.image.clone_from(&any.image);
            }

            if let Some(atlas_layout_handle) = atlas_layout {
                sprite.layout = Some(atlas_layout_handle.clone());
            } else if let Some(any) = any.as_ref() {
                sprite.layout.clone_from(&any.layout);
            }

            if field_angle == Angle::Any {
                let any = sprite.clone();
                for s in action.values_mut() {
                    if s.image.is_none() {
                        s.image.clone_from(&any.image);
                    }
                    if s.layout.is_none() {
                        s.layout.clone_from(&any.layout);
                    }
                }
            }
        }
    }
}

/// Convert actor/action to number id using xxh3_64
pub fn get_act_id(act: &str) -> u64 {
    xxh3_64(act.as_bytes())
}
