use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_2dviewangle::{
    Animation2D, Dynamic2DView, TextureViewCollections, View2DAnglePlugin, ViewChanged,
    ViewAngle,
};
use std::collections::HashMap;

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

const SPRITE_LEN: usize = 3;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

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
        .add_systems(Update, (animate_sprite, input))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animation2d: ResMut<Animation2D>,
) {
    let front_image = asset_server.load("frog_idle_front.png");
    let back_image = asset_server.load("frog_idle_back.png");
    let left_image = asset_server.load("frog_idle_left.png");

    let front_atlas = TextureAtlas::from_grid(front_image, Vec2::new(16., 16.), 1, 3, None, None);
    let back_atlas = TextureAtlas::from_grid(back_image, Vec2::new(16., 16.), 1, 3, None, None);
    let left_atlas = TextureAtlas::from_grid(left_image, Vec2::new(16., 16.), 1, 3, None, None);

    let front_handle = texture_atlases.add(front_atlas);
    let back_handle = texture_atlases.add(back_atlas);
    let left_handle = texture_atlases.add(left_atlas);

    // Add handles of different views to plugin's resource
    animation2d.insert(
        Actor::Frog as u64,
        HashMap::from([(
            Action::Idle as u16,
            TextureViewCollections {
                front: Some(front_handle.clone()),
                back: Some(back_handle.clone()),
                left: Some(left_handle.clone()),
                ..default()
            },
        )]),
    );

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: front_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform::from_scale(Vec3::splat(10.)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        // Specify actor for entity
        Dynamic2DView {
            actor: Actor::Frog as u64,
            ..default()
        },
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % SPRITE_LEN;
        }
    }
}

fn input(
    kb_input: Res<Input<KeyCode>>,
    mut actors: Query<(&mut Dynamic2DView, Entity)>,
    mut action_event: EventWriter<ViewChanged>,
) {
    for (mut act, e) in actors.iter_mut() {
        let mut action = act.action;
        let mut direction = act.direction;

        // Update action and direction of actor
        if kb_input.any_pressed([KeyCode::Left, KeyCode::A]) {
            action = Action::Idle as u16;
            direction = ViewAngle::Left;
        } else if kb_input.any_pressed([KeyCode::Right, KeyCode::D]) {
            action = Action::Idle as u16;
            direction = ViewAngle::Right;
        } else if kb_input.any_pressed([KeyCode::Up, KeyCode::W]) {
            action = Action::Idle as u16;
            direction = ViewAngle::Back;
        } else if kb_input.any_pressed([KeyCode::Down, KeyCode::S]) {
            action = Action::Idle as u16;
            direction = ViewAngle::Front;
        }

        if action != act.action || direction != act.direction {
            act.action = action;
            act.direction = direction;
            // Send event to change to another sprite sheet of another view
            action_event.send(ViewChanged { entity: e });
        }
    }
}
