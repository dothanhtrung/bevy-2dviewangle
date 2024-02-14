// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use bevy::asset::{Assets, Handle};
use bevy::prelude::{
    EventReader, Query, Res, ResMut, StandardMaterial, TextureAtlasSprite, Time, Transform,
};
use bevy::sprite::TextureAtlas;
use bevy_sprite3d::AtlasSprite3dComponent;

use crate::component::*;

pub fn view_changed_event(
    mut events: EventReader<ViewChanged>,
    mut sprites: Query<(
        &mut DynamicActor,
        &mut Transform,
        Option<&Handle<StandardMaterial>>,
        Option<&mut Handle<TextureAtlas>>,
    )>,
    #[cfg(feature = "3d")]
    mut mats: ResMut<Assets<StandardMaterial>>,
    atlases: Res<Assets<TextureAtlas>>,
    animation2d: Res<ActorsTextures>,
) {
    for event in events.read() {
        if let Ok(s) = sprites.get_mut(event.entity) {
            let mut view = s.0;
            let mut transform = s.1;

            let action = view.action;
            let mut atlas = match view.angle {
                Angle::Front => &animation2d[&view.actor][&action].front,
                Angle::Back => &animation2d[&view.actor][&action].back,
                Angle::Left => &animation2d[&view.actor][&action].left,
                Angle::Right => &animation2d[&view.actor][&action].right,
                Angle::FrontLeft => &animation2d[&view.actor][&action].front_left,
                Angle::FrontRight => &animation2d[&view.actor][&action].front_right,
                Angle::BackLeft => &animation2d[&view.actor][&action].back_left,
                Angle::BackRight => &animation2d[&view.actor][&action].back_right,
            };

            if view.flipped {
                transform.rotate_y(std::f64::consts::PI as f32);
                view.flipped = false;
            }

            if atlas.is_none() {
                atlas = get_opposite_view(&animation2d[&view.actor][&action], view.angle);
                if atlas.is_some() {
                    transform.rotate_y(std::f64::consts::PI as f32);
                    view.flipped = true;
                }
            }

            #[cfg(feature = "3d")]
            if let (Some(mat), Some(atlas)) = (s.2, atlas) {
                let material = mats.get_mut(&*mat).unwrap();
                if let Some(atlas) = atlases.get(atlas) {
                    material.base_color_texture = Some(atlas.texture.clone());
                }
            }

            #[cfg(feature = "2d")]
            if let (Some(mut handle), Some(atlas)) = (s.3, atlas) {
                *handle = atlas.clone();
            }
        }
    }
}

fn get_opposite_view(texture: &ViewTextures, direction: Angle) -> &Option<Handle<TextureAtlas>> {
    match direction {
        Angle::Left => &texture.right,
        Angle::Right => &texture.left,
        Angle::FrontLeft => &texture.front_right,
        Angle::FrontRight => &texture.front_left,
        Angle::BackLeft => &texture.back_right,
        Angle::BackRight => &texture.back_left,
        _ => &None,
    }
}

pub fn dynamic_actor_animate(
    time: Res<Time>,
    atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut DynamicActor,
        Option<&mut AtlasSprite3dComponent>,
        Option<&mut TextureAtlasSprite>,
        Option<&Handle<TextureAtlas>>,
    )>,
) {
    for (mut actor, sprite3d, sprite2d, atlas_handle) in &mut query {
        if let Some(ref mut animation_timer) = actor.animation_timer {
            animation_timer.tick(time.delta());
            if animation_timer.just_finished() {
                #[cfg(feature = "3d")]
                if let Some(mut sprite) = sprite3d {
                    sprite.index = (sprite.index + 1) % sprite.atlas.len();
                }

                #[cfg(feature = "2d")]
                if let (Some(mut sprite), Some(atlas_handle)) = (sprite2d, atlas_handle) {
                    if let Some(atlas) = atlases.get(atlas_handle) {
                        let len = atlas.textures.len();
                        sprite.index = (sprite.index + 1) % len;
                    }
                }
            }
        }
    }
}
