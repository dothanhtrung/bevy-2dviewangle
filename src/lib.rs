// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//! Bevy plugin to easier to switch texture base on view angles. Currently, support 8 view angles:
//!
//! * front
//! * back
//! * left
//! * right
//! * front_left
//! * front_right
//! * back_left
//! * back_right
//!
//! Examples
//! --------
//!
//! * [2d example](https://gitlab.com/kimtinh/bevy-2dviewangle/-/blob/master/examples/2d.rs)
//! * [3d example](https://gitlab.com/kimtinh/bevy-2dviewangle/-/blob/master/examples/3d.rs)

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
