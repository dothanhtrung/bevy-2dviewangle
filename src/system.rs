use bevy::asset::{Assets, Handle};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{EventReader, Query, Res, ResMut};
use bevy::sprite::TextureAtlas;

use crate::component::*;

#[cfg(feature = "3d")]
pub fn texture_event_3d(
    mut events: EventReader<ViewChanged>,
    mut sprites: Query<(&mut ActionDirection, &mut Handle<StandardMaterial>)>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    let mut entities = Vec::new();

    for event in events.iter() {
        if let Ok(mut e) = sprites.get_mut(event.entity) {
            entities.push(event.entity);
        }
    }

    for e in entities {
        if let Ok(e) = sprites.get_mut(e) {
            let mut material = mats.get_mut(&e.1).unwrap();
            let action = e.0.action;
            let atlas = match e.0.direction {
                ViewDirection::Front => &e.0.animation[&action].front,
                ViewDirection::Back => &e.0.animation[&action].back,
                ViewDirection::Left => &e.0.animation[&action].left,
                ViewDirection::Right => &e.0.animation[&action].right,
                ViewDirection::FrontLeft => &e.0.animation[&action].front_left,
                ViewDirection::FrontRight => &e.0.animation[&action].front_right,
                ViewDirection::BackLeft => &e.0.animation[&action].back_left,
                ViewDirection::BackRight => &e.0.animation[&action].back_right,
            };
            if let Some(atlas) = atlas {
                if let Some(atlas) = atlases.get(atlas) {
                    material.base_color_texture = Some(atlas.texture.clone());
                }
            }
        }
    }
}
