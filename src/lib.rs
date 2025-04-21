// Copyright 2024,2025 Trung Do <dothanhtrung@pm.me>

//! Bevy plugin to easier to manage and switch texture base on view angles.
//!
//!
//! ## Quickstart
//!
//! ```rust
//! // Struct to store sprite sheet
//! use bevy::prelude::*;
//! use bevy_2dviewangle::{Angle, View2dActor, ViewChanged};
//! use bevy_2dviewangle::View2dCollection;
//!
//! #[derive(View2dCollection, Default)]
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
//!     mut actors: Query<(&mut View2dActor, Entity)>,
//!     mut action_event: EventWriter<ViewChanged>,
//! ) {
//!     for (mut act, e) in actors.iter_mut() {
//!         act.action = ActionMyAssets::Idle.into();
//!         act.angle = Angle::Right;
//!         action_event.write(ViewChanged { entity: e });
//!     }
//! }
//! ```
//!
//! Please see in [examples](./examples) for more detail.
//!
//! This plugin can work with [bevy_asset_loader](https://crates.io/crates/bevy_asset_loader) too:
//!
//! ```rust
//! use bevy::prelude::{Handle, Image, Resource, TextureAtlasLayout};
//! use bevy_asset_loader::prelude::AssetCollection;
//! use bevy_2dviewangle::View2dCollection;
//!
//! #[derive(AssetCollection, View2dCollection, Resource)]
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
//!     #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 1, rows = 3))]
//!     #[textureview(angle = "any")]
//!     pub front_layout: Handle<TextureAtlasLayout>,
//! }
//! ```

use bevy::{
    app::{App, Plugin, Update},
    prelude::on_event,
};

use bevy::prelude::{in_state, IntoScheduleConfigs, States};

pub use component::*;
use system::*;

mod component;
mod system;

macro_rules! plugin_systems {
    () => {
        (
            view_changed_event.run_if(on_event::<ViewChanged>),
            dynamic_actor_animate,
        )
    };
}

/// The main plugin

#[derive(Default)]
pub struct View2DAnglePlugin<T>
where
    T: States,
{
    /// List of game state that this plugin will run in
    pub states: Vec<T>,
}

impl<T> Plugin for View2DAnglePlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        app.add_event::<ViewChanged>()
            .add_event::<LastFrame>()
            .insert_resource(ActorSpriteSheets::default());
        if self.states.is_empty() {
            app.add_systems(Update, plugin_systems!());
        } else {
            for state in self.states.iter() {
                app.add_systems(Update, plugin_systems!().run_if(in_state(state.clone())));
            }
        }
    }
}

impl<T> View2DAnglePlugin<T>
where
    T: States,
{
    pub fn new(states: Vec<T>) -> Self {
        Self { states }
    }
}

#[derive(States, Clone, Debug, Hash, Eq, PartialEq)]
pub enum DummyState {}

/// Use this if you don't care to state and want this plugin's systems run all the time.
#[derive(Default)]
pub struct View2DAnglePluginAnyState;

impl View2DAnglePluginAnyState {
    pub fn any() -> View2DAnglePlugin<DummyState> {
        View2DAnglePlugin::new(Vec::new())
    }
}
