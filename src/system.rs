// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use bevy::asset::Assets;
use bevy::prelude::{Entity, EventReader, EventWriter, Query, Res, Sprite, Time};
use bevy::sprite::TextureAtlasLayout;

use crate::component::*;

/// Check on `ViewChanged` event and change to corresponding spritesheet.
/// If spritesheet for an angle does not exist, it will try to flip the spritesheet of the opposite angle.
/// If the opposite is not available, spritesheet will not change.
pub(crate) fn view_changed_event(
    mut events: EventReader<ViewChanged>,
    mut sprites: Query<(&mut View2dActor, &mut Sprite)>,
    animation2d: Res<ActorSpriteSheets>,
) {
    for event in events.read() {
        if let Ok((mut view, mut sprite)) = sprites.get_mut(event.entity) {
            let action = view.action;
            let mut viewsprite = animation2d[&view.actor][&action].get(&view.angle);

            if view.flipped {
                sprite.flip_x = false;
                view.flipped = false;
            }

            if viewsprite.is_none() {
                viewsprite = get_opposite_view(&animation2d[&view.actor][&action], view.angle);
                if viewsprite.is_some() {
                    sprite.flip_x = true;
                    view.flipped = true;
                } else {
                    return;
                }
            }

            let viewsprite = viewsprite.unwrap();

            if viewsprite.image.is_some() {
                sprite.image = viewsprite.image.as_ref().unwrap().clone();
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.layout = viewsprite.layout.as_ref().unwrap().clone();
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
    time: Res<Time>,
    atlases: Res<Assets<TextureAtlasLayout>>,
    mut query: Query<(&mut View2dActor, &mut Sprite, Entity)>,
    mut event: EventWriter<LastFrame>,
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
                                        event.send(LastFrame { entity });
                                    }
                                }
                            }
                        }

                        atlas.index = (atlas.index + 1) % layout.textures.len();
                    }
                }
            }
        }
    }
}
