// Copyright 2024,2025 Trung Do <dothanhtrung@pm.me>

use bevy::asset::Assets;
use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, Res, Sprite, TextureAtlasLayout, Time};

use crate::component::*;

const ACTION_ANY: u64 = 0;
const ACTOR_ANY: u64 = 0;

/// Check on `ViewChanged` event and change to corresponding spritesheet.
/// If spritesheet for an angle does not exist, it will try to flip the spritesheet of the opposite angle.
/// If the opposite is not available, spritesheet will not change.
pub(crate) fn view_changed_event(
    mut events: MessageReader<ViewChanged>,
    mut sprites: Query<(&mut View2dActor, &mut Sprite)>,
    animation2d: Res<ActorSpriteSheets>,
) {
    for event in events.read() {
        if let Ok((mut view, mut sprite)) = sprites.get_mut(event.entity) {
            let mut action = view.action;
            let mut actor = view.actor;

            let mut viewsprite = None;
            for _actor in [&view.actor, &ACTOR_ANY] {
                if let Some(actor_val) = animation2d.get(_actor) {
                    for _action in [&view.action, &ACTION_ANY] {
                        if let Some(action_val) = actor_val.get(_action) {
                            viewsprite = action_val.get(&view.angle);
                            actor = *_actor;
                            action = *_action;
                            break;
                        }
                    }
                }
            }

            if view.flipped {
                sprite.flip_x = false;
                view.flipped = false;
            }

            // TODO: Clean code
            if viewsprite.is_none() {
                viewsprite = get_opposite_view(&animation2d[&actor][&action], view.angle);
                if viewsprite.is_some() {
                    sprite.flip_x = true;
                    view.flipped = true;
                }
            }
            if viewsprite.is_none() {
                viewsprite = animation2d[&actor][&action].get(&Angle::Any);
                if viewsprite.is_none() {
                    viewsprite = get_opposite_view(&animation2d[&actor][&action], Angle::Any);
                    if viewsprite.is_some() {
                        sprite.flip_x = true;
                        view.flipped = true;
                    } else {
                        return;
                    }
                }
            }

            let viewsprite = viewsprite.unwrap();

            if viewsprite.image.is_some() {
                sprite.image = viewsprite.image.as_ref().unwrap().clone();
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if let Some(view_atlas) = &viewsprite.layout {
                        atlas.layout = view_atlas.clone();
                    }
                }
            }
        }
    }
}

fn get_opposite_view(texture: &AngleSpriteSheets, direction: Angle) -> Option<&SpriteSheet> {
    match direction {
        Angle::Left => texture.get(&Angle::Right),
        Angle::Right => texture.get(&Angle::Left),
        Angle::FrontLeft => texture.get(&Angle::FrontRight),
        Angle::FrontRight => texture.get(&Angle::FrontLeft),
        Angle::BackLeft => texture.get(&Angle::BackRight),
        Angle::BackRight => texture.get(&Angle::BackLeft),
        _ => None,
    }
}

pub(crate) fn dynamic_actor_animate(
    mut commands: Commands,
    time: Res<Time>,
    atlases: Res<Assets<TextureAtlasLayout>>,
    mut query: Query<(&mut View2dActor, &mut Sprite, Entity)>,
    mut event: MessageWriter<ViewChanged>,
) {
    for (mut actor, mut sprite, entity) in &mut query {
        if let Some(ref mut animation_timer) = actor.animation_timer {
            animation_timer.tick(time.delta());
            if animation_timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if let Some(layout) = atlases.get(&atlas.layout) {
                        for notify in &actor.notify {
                            match *notify {
                                Notification::LastFrame => {
                                    if atlas.index == layout.textures.len() - 1 {
                                        commands.trigger(LastFrame { entity });
                                    }
                                }
                            }
                        }

                        if atlas.index == layout.textures.len() - 1 {
                            if let Some(next_action) = actor.next_action.first() {
                                actor.action = *next_action;
                                event.write(ViewChanged { entity });
                                actor.next_action.remove(0);
                            }
                        }
                        atlas.index = (atlas.index + 1) % layout.textures.len();
                    }
                }
            }
        }
    }
}
