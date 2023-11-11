use bevy::asset::{Assets, Handle};
use bevy::prelude::{EventReader, Query, Res, ResMut, StandardMaterial, Transform};
use bevy::sprite::TextureAtlas;

use crate::component::*;

#[cfg(feature = "3d")]
pub fn texture_event_3d(
    mut events: EventReader<ViewChanged>,
    mut sprites: Query<(&mut Dynamic2DView, &mut Handle<StandardMaterial>, &mut Transform)>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    atlases: Res<Assets<TextureAtlas>>,
    animation2d: Res<Animation2D>,
) {
    for event in events.iter() {
        if let Ok(s) = sprites.get_mut(event.entity) {
            let mut view = s.0;
            let mat = s.1;
            let mut transform = s.2;

            let mut material = mats.get_mut(&*mat).unwrap();
            let action = view.action;
            let atlas = match view.direction {
                ViewDirection::Front => &animation2d[&view.actor][&action].front,
                ViewDirection::Back => &animation2d[&view.actor][&action].back,
                ViewDirection::Left => &animation2d[&view.actor][&action].left,
                ViewDirection::Right => &animation2d[&view.actor][&action].right,
                ViewDirection::FrontLeft => &animation2d[&view.actor][&action].front_left,
                ViewDirection::FrontRight => &animation2d[&view.actor][&action].front_right,
                ViewDirection::BackLeft => &animation2d[&view.actor][&action].back_left,
                ViewDirection::BackRight => &animation2d[&view.actor][&action].back_right,
            };

            if view.flipped {
                transform.rotate_y(std::f64::consts::PI as f32);
                view.flipped = false;
            }

            if let Some(atlas) = atlas {
                if let Some(atlas) = atlases.get(atlas) {
                    material.base_color_texture = Some(atlas.texture.clone());
                }
            } else if let Some(atlas) = get_opposite_view(&animation2d[&view.actor][&action], view.direction) {
                if let Some(atlas) = atlases.get(atlas) {
                    material.base_color_texture = Some(atlas.texture.clone());
                }

                transform.rotate_y(std::f64::consts::PI as f32);
                view.flipped = true;
            }
        }
    }
}

fn get_opposite_view(texture: &TextureViewCollections, direction: ViewDirection) -> &Option<Handle<TextureAtlas>> {
    match direction {
        ViewDirection::Left => &texture.right,
        ViewDirection::Right => &texture.left,
        ViewDirection::FrontLeft => &texture.front_right,
        ViewDirection::FrontRight => &texture.front_left,
        ViewDirection::BackLeft => &texture.back_right,
        ViewDirection::BackRight => &texture.back_left,
        _ => &None,
    }
}
