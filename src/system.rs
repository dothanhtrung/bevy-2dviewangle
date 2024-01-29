// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use bevy::asset::{Assets, Handle};
use bevy::prelude::{EventReader, Query, Res, ResMut, StandardMaterial, Transform};
use bevy::sprite::TextureAtlas;

use crate::component::*;

pub fn view_changed_event(
    mut events: EventReader<ViewChanged>,
    mut sprites: Query<(
        &mut Dynamic2DView,
        &mut Transform,
        Option<&Handle<StandardMaterial>>,
        Option<&mut Handle<TextureAtlas>>,
    )>,
    #[cfg(feature = "3d")]
    mut mats: ResMut<Assets<StandardMaterial>>,
    atlases: Res<Assets<TextureAtlas>>,
    animation2d: Res<Animation2D>,
) {
    for event in events.read() {
        if let Ok(s) = sprites.get_mut(event.entity) {
            let mut view = s.0;
            let mut transform = s.1;

            let action = view.action;
            let mut atlas = match view.direction {
                ViewAngle::Front => &animation2d[&view.actor][&action].front,
                ViewAngle::Back => &animation2d[&view.actor][&action].back,
                ViewAngle::Left => &animation2d[&view.actor][&action].left,
                ViewAngle::Right => &animation2d[&view.actor][&action].right,
                ViewAngle::FrontLeft => &animation2d[&view.actor][&action].front_left,
                ViewAngle::FrontRight => &animation2d[&view.actor][&action].front_right,
                ViewAngle::BackLeft => &animation2d[&view.actor][&action].back_left,
                ViewAngle::BackRight => &animation2d[&view.actor][&action].back_right,
            };

            if view.flipped {
                transform.rotate_y(std::f64::consts::PI as f32);
                view.flipped = false;
            }

            if atlas.is_none() {
                atlas = get_opposite_view(&animation2d[&view.actor][&action], view.direction);
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

fn get_opposite_view(
    texture: &TextureViewCollections,
    direction: ViewAngle,
) -> &Option<Handle<TextureAtlas>> {
    match direction {
        ViewAngle::Left => &texture.right,
        ViewAngle::Right => &texture.left,
        ViewAngle::FrontLeft => &texture.front_right,
        ViewAngle::FrontRight => &texture.front_left,
        ViewAngle::BackLeft => &texture.back_right,
        ViewAngle::BackRight => &texture.back_left,
        _ => &None,
    }
}
