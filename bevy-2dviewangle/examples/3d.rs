use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::camera::Exposure;
use bevy::window::WindowResolution;
use bevy_sprite3d::{Sprite3d, Sprite3dParams, Sprite3dPlugin};

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

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
enum GameState {
    #[default]
    Loading,
    Ready,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("3D demo"),
                        resolution: WindowResolution::new(256., 256.),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // Add the plugin
        .add_plugins(View2DAnglePlugin)
        .add_plugins(Sprite3dPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, load_texture)
        .add_systems(Update, setup.run_if(in_state(GameState::Loading)))
        .add_systems(Update, input.run_if(in_state(GameState::Ready)))
        .run();
}

fn load_texture(
    asset_server: Res<AssetServer>,
    mut animation2d: ResMut<ActorsTextures>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let front_image = asset_server.load("frog_idle_front.png");
    let back_image = asset_server.load("frog_idle_back.png");
    let left_image = asset_server.load("frog_idle_left.png");

    let front_atlas = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 1, 3, None, None);
    let back_atlas = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 1, 3, None, None);
    let left_atlas = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 1, 3, None, None);

    let front_handle = atlases.add(front_atlas);
    let back_handle = atlases.add(back_atlas);
    let left_handle = atlases.add(left_atlas);

    // Add handles of different views to plugin's resource
    animation2d.insert(
        Actor::Frog as u64,
        HashMap::from([(
            Action::Idle as u16,
            ViewTextures::from(vec![
                (
                    Angle::Front,
                    ViewSprite {
                        layout: front_handle.clone(),
                        image: front_image.clone(),
                    },
                ),
                (
                    Angle::Back,
                    ViewSprite {
                        layout: back_handle,
                        image: back_image,
                    },
                ),
                (
                    Angle::Left,
                    ViewSprite {
                        layout: left_handle,
                        image: left_image,
                    },
                ),
            ]),
        )]),
    );
}

fn setup(
    mut commands: Commands,
    animation2d: Res<ActorsTextures>,
    mut sprite3d_params: Sprite3dParams,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let front_handle = animation2d
        .get(&(Actor::Frog as u64))
        .unwrap()
        .get(&(Action::Idle as u16))
        .unwrap()
        .get(&Angle::Front)
        .unwrap();

    next_state.set(GameState::Ready);

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4., 10., 4.),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 2.5, 3.).looking_at(Vec3::Y, Vec3::Y),
        exposure: Exposure::INDOOR,
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: sprite3d_params.meshes.add(Circle::new(4.0)),
        material: sprite3d_params.materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // Spawn frog
    let texture_atlas = TextureAtlas {
        layout: front_handle.layout.clone(),
        index: 0,
    };
    commands.spawn((
        Sprite3d {
            image: front_handle.image.clone(),
            pixels_per_metre: 8.,
            transform: Transform {
                translation: Vec3::new(0., 0.85, 0.),
                ..default()
            },
            ..default()
        }
        .bundle_with_atlas(&mut sprite3d_params, texture_atlas),
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
