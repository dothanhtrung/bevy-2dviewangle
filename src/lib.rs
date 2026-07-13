#![doc=include_str!("../README.md")]

pub mod component;
pub mod system;

pub use crate::component::*;
use crate::system::{
    animated_timer,
    animating,
    view_changed_event,
};
use bevy::prelude::{
    App,
    IntoScheduleConfigs,
    Plugin,
    States,
    Update,
    in_state,
    on_message,
};

macro_rules! plugin_systems {
    () => {
        (view_changed_event.run_if(on_message::<ViewChanged>), animated_timer)
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
        app.register_type::<View2dActor>()
            .add_message::<ViewChanged>()
            .insert_resource(ActorSpriteSheets::default())
            .add_observer(animating);
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

    pub fn any() -> Self {
        Self { states: Vec::new() }
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
