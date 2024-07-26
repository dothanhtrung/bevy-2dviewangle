use bevy::prelude::*;
use bevy::render::camera::Exposure;
use bevy::window::WindowResolution;
use bevy_sprite3d::{Sprite3d, Sprite3dParams, Sprite3dPlugin};

use bevy_2dviewangle::{ActorSpriteSheets, Angle, View2DAnglePluginNoState, View2dActor};

use crate::common::{input, ActionMyAssets, ActorMyAssets, MyAssets};

mod common;

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
enum GameState {
    #[default]
    Loading,
    Ready,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("3D demo"),
                resolution: WindowResolution::new(256., 256.),
                ..default()
            }),
            ..default()
        }))
        // Add the plugin
        .add_plugins(View2DAnglePluginNoState)
        .add_plugins(Sprite3dPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, load_texture)
        .add_systems(Update, setup.run_if(in_state(GameState::Loading)))
        .add_systems(Update, input.run_if(in_state(GameState::Ready)))
        .run();
}

fn load_texture(
    asset_server: Res<AssetServer>,
    mut animation2d: ResMut<ActorSpriteSheets>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 1, 3, None, None);
    let my_assets = MyAssets {
        idle_front: asset_server.load("frog_idle_front.png"),
        idle_back: asset_server.load("frog_idle_back.png"),
        idle_left: asset_server.load("frog_idle_left.png"),
        layout: atlases.add(layout),
    };

    // Load into collection
    animation2d.load_asset_loader(&my_assets);
}

fn setup(
    mut commands: Commands,
    animation2d: Res<ActorSpriteSheets>,
    mut sprite3d_params: Sprite3dParams,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let front_handle = animation2d
        .get(&(ActorMyAssets::Frog.into()))
        .unwrap()
        .get(&(ActionMyAssets::Idle.into()))
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
        layout: front_handle.layout.as_ref().unwrap().clone(),
        index: 0,
    };
    commands.spawn((
        Sprite3d {
            image: front_handle.image.as_ref().unwrap().clone(),
            pixels_per_metre: 8.,
            transform: Transform {
                translation: Vec3::new(0., 0.85, 0.),
                ..default()
            },
            ..default()
        }
        .bundle_with_atlas(&mut sprite3d_params, texture_atlas),
        // Specify actor for entity
        View2dActor {
            actor: ActorMyAssets::Frog.into(),
            animation_timer: Some(Timer::from_seconds(0.25, TimerMode::Repeating)),
            ..default()
        },
    ));
}
