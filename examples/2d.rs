use std::collections::HashMap;

use bevy::prelude::*;
use bevy::window::WindowResolution;

use bevy_2dviewangle::{
    ActorsTextures, Angle, DynamicActor, View2DAnglePlugin, ViewChanged, ViewSprite, ViewTextures,
};

// There may be many actors: player, animal, npc, ...
#[repr(u64)]
enum Actor {
    Frog,
}

// Each actor may have many actions: idle, walk, run, fight, ...
#[repr(u16)]
enum Action {
    Idle,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("2D demo"),
                        resolution: WindowResolution::new(256., 256.),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // Add the plugin
        .add_plugins(View2DAnglePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, input)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut animation2d: ResMut<ActorsTextures>,
) {
    let front_image = asset_server.load("frog_idle_front.png");
    let back_image = asset_server.load("frog_idle_back.png");
    let left_image = asset_server.load("frog_idle_left.png");

    let front_atlas = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 1, 3, None, None);
    let back_atlas = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 1, 3, None, None);
    let left_atlas = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 1, 3, None, None);

    let front_handle = texture_atlases.add(front_atlas);
    let back_handle = texture_atlases.add(back_atlas);
    let left_handle = texture_atlases.add(left_atlas);

    // Add handles of different views to plugin's resource
    animation2d.insert(
        Actor::Frog as u64,
        HashMap::from([(
            Action::Idle as u16,
            ViewTextures {
                front: Some(ViewSprite {
                    layout: front_handle.clone(),
                    image: front_image.clone(),
                }),
                back: Some(ViewSprite {
                    layout: back_handle,
                    image: back_image,
                }),
                left: Some(ViewSprite {
                    layout: left_handle,
                    image: left_image,
                }),
                ..default()
            },
        )]),
    );

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture: front_image.clone(),
            atlas: TextureAtlas {
                layout: front_handle.clone(),
                ..default()
            },
            transform: Transform::from_scale(Vec3::splat(10.)),
            ..default()
        },
        // Specify actor for entity
        DynamicActor {
            actor: Actor::Frog as u64,
            animation_timer: Some(Timer::from_seconds(0.25, TimerMode::Repeating)),
            ..default()
        },
    ));
}

fn input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut actors: Query<(&mut DynamicActor, Entity)>,
    mut action_event: EventWriter<ViewChanged>,
) {
    for (mut act, e) in actors.iter_mut() {
        let mut action = act.action;
        let mut direction = act.angle;

        // Update action and direction of actor
        if kb_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            action = Action::Idle as u16;
            direction = Angle::Left;
        } else if kb_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            action = Action::Idle as u16;
            direction = Angle::Right;
        } else if kb_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            action = Action::Idle as u16;
            direction = Angle::Back;
        } else if kb_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            action = Action::Idle as u16;
            direction = Angle::Front;
        }

        if action != act.action || direction != act.angle {
            act.action = action;
            act.angle = direction;
            // Send event to change to another sprite sheet of another view
            action_event.send(ViewChanged { entity: e });
        }
    }
}
