use bevy::app::{App, Plugin, Update};

pub use component::*;
use system::*;

mod component;
mod system;

pub struct View2DAnglePlugin;

impl Plugin for View2DAnglePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ViewChanged>();

        #[cfg(feature = "3d")]
        app.add_systems(Update, texture_event_3d)
            .insert_resource(Animation2D::default());
    }
}
