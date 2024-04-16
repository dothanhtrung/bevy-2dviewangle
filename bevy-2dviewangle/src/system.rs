// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use bevy::asset::{Assets, Handle};
use bevy::prelude::{EventReader, Image, Query, Res, ResMut, StandardMaterial, Time, Transform};
use bevy::sprite::{TextureAtlas, TextureAtlasLayout};

use crate::component::*;

pub fn view_changed_event(
    mut events: EventReader<ViewChanged>,
    mut sprites: Query<(
        &mut DynamicActor,
        &mut Transform,
        Option<&Handle<StandardMaterial>>,
        Option<&mut Handle<Image>>,
    )>,
    #[cfg(feature = "3d")] mut mats: ResMut<Assets<StandardMaterial>>,
    animation2d: Res<ActorsTextures>,
) {
    for event in events.read() {
        if let Ok(s) = sprites.get_mut(event.entity) {
            let mut view = s.0;
            let mut transform = s.1;

            let action = view.action;
            let mut atlas = animation2d[&view.actor][&action].get(&view.angle);

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
                if atlas.image.is_some() {
                    let material = mats.get_mut(mat).unwrap();
                    material.base_color_texture = Some(atlas.image.as_ref().unwrap().clone());
                }
            }

            #[cfg(feature = "2d")]
            if let (Some(mut handle), Some(atlas)) = (s.3, atlas) {
                if atlas.image.is_some() {
                    *handle = atlas.image.as_ref().unwrap().clone();
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

pub fn dynamic_actor_animate(
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
