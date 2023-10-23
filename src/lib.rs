use bevy::app::{App, Plugin, Update};

mod component;
mod system;

use system::*;

pub struct View2DAnglePlugin;

impl Plugin for View2DAnglePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "3d")]
        app.add_systems(Update, texture_event_3d);
    }
}
