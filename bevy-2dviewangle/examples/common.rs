use bevy::asset::Handle;
use bevy::input::ButtonInput;
use bevy::prelude::{Entity, EventWriter, Image, KeyCode, Query, Res, TextureAtlasLayout};

use bevy_2dviewangle::{ActorsTexturesCollection, Angle, DynamicActor, ViewChanged};

// Struct to load spritesheet
#[derive(ActorsTexturesCollection, Default)]
pub struct MyAssets {
    #[textureview(actor = 0, action = 0, angle = "front", handle = "image")]
    pub idle_front: Handle<Image>,

    // If not specify actor/action, the previous value will be used
    #[textureview(angle = "back", handle = "image")]
    pub idle_back: Handle<Image>,

    #[textureview(angle = "left", handle = "image")]
    pub idle_left: Handle<Image>,

    // If angle is any, other angle which has not been defined will use this value
    #[textureview(angle = "any", handle = "atlas_layout")]
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut actors: Query<(&mut DynamicActor, Entity)>,
    mut action_event: EventWriter<ViewChanged>,
) {
    for (mut act, e) in actors.iter_mut() {
        let mut action = act.action;
        let mut direction = act.angle;

        // Update action id and direction of actor
        if kb_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            action = 0;
            direction = Angle::Left;
        } else if kb_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            action = 0;
            direction = Angle::Right;
        } else if kb_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            action = 0;
            direction = Angle::Back;
        } else if kb_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            action = 0;
            direction = Angle::Front;
        }

        if action != act.action || direction != act.angle {
            act.action = action;
            act.angle = direction;
            // Send event to change to sprite sheet to another view
            action_event.send(ViewChanged { entity: e });
        }
    }
}
