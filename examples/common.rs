use bevy::asset::Handle;
use bevy::input::ButtonInput;
use bevy::prelude::{Entity, EventWriter, Image, KeyCode, Query, Res, TextureAtlasLayout};

use bevy_2dviewangle::{ActorsTexturesCollection, DynamicActor, ViewChanged};

// Struct to load spritesheet
// The derive macro will provide these two enums too:
// `enum ActorMyAssets { Frog }` and `enum ActionMyAssets { Idle }`
#[derive(ActorsTexturesCollection, Default)]
pub struct MyAssets {
    #[textureview(actor = "frog", action = "idle", angle = "front")]
    pub idle_front: Handle<Image>,

    // If not specify actor/action, the previous value will be used
    #[textureview(angle = "back")]
    pub idle_back: Handle<Image>,

    // If the angle "right" is not defined, it will be flipped base on the angle "left" image
    #[textureview(angle = "left")]
    pub idle_left: Handle<Image>,

    // If angle is any, other angle which has not been defined will use this value
    #[textureview(angle = "any")]
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
            action = ActionMyAssets::Idle as u16;
            direction = Angle::Left;
        } else if kb_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            action = ActionMyAssets::Idle as u16;
            direction = Angle::Right;
        } else if kb_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            action = ActionMyAssets::Idle as u16;
            direction = Angle::Back;
        } else if kb_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            action = ActionMyAssets::Idle as u16;
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
