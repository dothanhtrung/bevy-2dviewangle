use bevy::asset::{Assets, Handle};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{EventReader, Query, Res, ResMut};
use bevy::sprite::TextureAtlas;

use crate::component::*;

#[cfg(feature = "3d")]
pub fn texture_event_3d(
    mut events: EventReader<ViewChanged>,
    mut sprites: Query<(&mut ActorView, &mut Handle<StandardMaterial>)>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    atlases: Res<Assets<TextureAtlas>>,
    animation2d: Res<Animation2D>,
) {
    let mut entities = Vec::new();

    for event in events.iter() {
        if let Ok(_) = sprites.get_mut(event.entity) {
            entities.push(event.entity);
        }
    }

    for e in entities {
        if let Ok(e) = sprites.get_mut(e) {
            let mut material = mats.get_mut(&e.1).unwrap();
            let action = e.0.action;
            let atlas = match e.0.direction {
                ViewDirection::Front => &animation2d[&e.0.actor][&action].front,
                ViewDirection::Back => &animation2d[&e.0.actor][&action].back,
                ViewDirection::Left => &animation2d[&e.0.actor][&action].left,
                ViewDirection::Right => &animation2d[&e.0.actor][&action].right,
                ViewDirection::FrontLeft => &animation2d[&e.0.actor][&action].front_left,
                ViewDirection::FrontRight => &animation2d[&e.0.actor][&action].front_right,
                ViewDirection::BackLeft => &animation2d[&e.0.actor][&action].back_left,
                ViewDirection::BackRight => &animation2d[&e.0.actor][&action].back_right,
            };
            if let Some(atlas) = atlas {
                if let Some(atlas) = atlases.get(atlas) {
                    material.base_color_texture = Some(atlas.texture.clone());
                }
            }
        }
    }
}
