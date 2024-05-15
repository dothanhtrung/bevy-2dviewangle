// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//! Bevy plugin to easier to manage and switch texture base on view angles.
//!
//!
//! ## Quickstart
//!
//! ```rust
//! // Struct to store sprite sheet
//! use bevy::prelude::*;
//! use bevy_2dviewangle_macro::ActorsTexturesCollection;
//! use bevy_2dviewangle::{Angle, DynamicActor, ViewChanged};
//!
//! #[derive(ActorsTexturesCollection, Default)]
//! struct MyAssets {
//!     #[textureview(actor = "player", action = "idle", angle = "front")]
//!     pub idle_front: Handle<Image>,
//!
//!     // If not specify actor/action, the previous value will be used
//!     #[textureview(angle = "back")]
//!     pub idle_back: Handle<Image>,
//!
//!     // If the angle "right" is not defined, it will be flipped base on the angle "left" image
//!     #[textureview(angle = "left")]
//!     pub idle_left: Handle<Image>,
//!
//!     // If angle is any, other angle which has not been defined will use this value
//!     #[textureview(angle = "any")]
//!     pub idle_any_layout: Handle<TextureAtlasLayout>,
//! }
//!
//! // Change the sprite sheet by sending event
//! fn switch_sprite(
//!     mut actors: Query<(&mut DynamicActor, Entity)>,
//!     mut action_event: EventWriter<ViewChanged>,
//! ) {
//!     for (mut act, e) in actors.iter_mut() {
//!         act.action = Action::Idle;
//!         act.angle = Angle::Right;
//!         action_event.send(ViewChanged { entity: e });
//!     }
//! }
//! ```
//!
//! Please see in [examples](./examples) for more detail.
//!
//! This plugin can work with [bevy_asset_loader](https://crates.io/crates/bevy_asset_loader) too:
//!
//! ```rust
//! #[derive(AssetCollection, ActorsTexturesCollection, Resource)]
//! pub struct MyAssets {
//!     #[asset(path = "frog_idle_front.png")]
//!     #[textureview(actor = "frog", action = "idle", angle = "front")]
//!     pub idle_front: Handle<Image>,
//!
//!     #[asset(path = "frog_idle_back.png")]
//!     #[textureview(angle = "back")]
//!     pub idle_back: Handle<Image>,
//!
//!     #[asset(path = "frog_idle_left.png")]
//!     #[textureview(angle = "left")]
//!     pub idle_left: Handle<Image>,
//!
//!     #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 1, rows = 3))]
//!     #[textureview(angle = "any")]
//!     pub front_layout: Handle<TextureAtlasLayout>,
//! }
//! ```

use bevy::app::{App, Plugin, Update};

pub use component::*;
use system::*;

mod component;
mod system;

pub struct View2DAnglePlugin;

impl Plugin for View2DAnglePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ViewChanged>()
            .add_systems(Update, (view_changed_event, dynamic_actor_animate))
            .insert_resource(ActorsTextures::default());
    }
}
