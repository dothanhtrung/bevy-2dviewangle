// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use bevy::asset::{Assets, Handle};
use bevy::prelude::{EventReader, Image, Query, Res, StandardMaterial, Time, Transform};
use bevy::sprite::{TextureAtlas, TextureAtlasLayout};

#[cfg(feature = "3d")]
use bevy::prelude::ResMut;

use crate::component::*;

/// Check on `ViewChanged` event and change to corresponding spritesheet.
/// If spritesheet for an angle does not exist, it will try to flip the spritesheet of the opposite angle.
/// If the opposite is not available, spritesheet will not change.
pub(crate) fn view_changed_event(
    mut events: EventReader<ViewChanged>,
    mut sprites: Query<(
        &mut DynamicActor,
        &mut Transform,
        Option<&Handle<StandardMaterial>>,
        Option<&mut Handle<Image>>,
        Option<&mut Handle<TextureAtlasLayout>>,
    )>,
    #[cfg(feature = "3d")] mut mats: ResMut<Assets<StandardMaterial>>,
    animation2d: Res<ActorsTextures>,
) {
    for event in events.read() {
        if let Ok((mut view, mut transform, _mat, _handle, atlas_layout)) = sprites.get_mut(event.entity) {
            let action = view.action;
            let mut viewsprite = animation2d[&view.actor][&action].get(&view.angle);

            if view.flipped {
                transform.rotate_y(std::f64::consts::PI as f32);
                view.flipped = false;
            }

            if viewsprite.is_none() {
                viewsprite = get_opposite_view(&animation2d[&view.actor][&action], view.angle);
                if viewsprite.is_some() {
                    transform.rotate_y(std::f64::consts::PI as f32);
                    view.flipped = true;
                }
            }

            if viewsprite.is_none() {
                return;
            }

            let viewsprite = viewsprite.unwrap();

            #[cfg(feature = "3d")]
            if let Some(mat) = _mat {
                if viewsprite.image.is_some() {
                    let material = mats.get_mut(mat).unwrap();
                    material.base_color_texture = Some(viewsprite.image.as_ref().unwrap().clone());
                }
            }

            #[cfg(feature = "2d")]
            if let Some(mut handle) = _handle {
                if viewsprite.image.is_some() {
                    *handle = viewsprite.image.as_ref().unwrap().clone();
                }
            }
            if let Some(mut atlas_layout) = atlas_layout {
                if viewsprite.layout.is_some() {
                    *atlas_layout = viewsprite.layout.as_ref().unwrap().clone();
                }
            }
        }
    }
}

fn get_opposite_view(texture: &ViewTextures, direction: Angle) -> Option<&ViewSprite> {
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
    mut query: Query<(&mut DynamicActor, Option<&mut TextureAtlas>)>,
) {
    for (mut actor, texture_atlas) in &mut query {
        if let Some(ref mut animation_timer) = actor.animation_timer {
            animation_timer.tick(time.delta());
            if animation_timer.just_finished() {
                if let Some(mut atlas) = texture_atlas {
                    if let Some(layout) = atlases.get(&atlas.layout) {
                        atlas.index = (atlas.index + 1) % layout.textures.len();
                    }
                }
            }
        }
    }
}
