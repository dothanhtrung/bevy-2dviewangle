// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use bevy::app::{App, Plugin, Update};

pub use component::*;
use system::*;

mod component;
mod system;

pub struct View2DAnglePlugin;

impl Plugin for View2DAnglePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ViewChanged>();

        app.add_systems(Update, view_changed_event)
            .insert_resource(Animation2D::default());
    }
}
