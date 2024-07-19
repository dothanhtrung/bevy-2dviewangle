use bevy::prelude::*;
use bevy::window::WindowResolution;

use bevy_2dviewangle::{ActorsTextures, DynamicActor, View2DAnglePlugin};

use crate::common::{input, ActorMyAssets, MyAssets};

mod common;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("2D demo"),
                resolution: WindowResolution::new(256., 256.),
                ..default()
            }),
            ..default()
        }))
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
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 1, 3, None, None);
    let my_assets = MyAssets {
        idle_front: asset_server.load("frog_idle_front.png"),
        idle_back: asset_server.load("frog_idle_back.png"),
        idle_left: asset_server.load("frog_idle_left.png"),
        layout: texture_atlases.add(layout),
    };

    // Load into collection
    animation2d.load_asset_loader(&my_assets);

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: my_assets.idle_front.clone(),
            transform: Transform::from_scale(Vec3::splat(10.)),
            ..default()
        },
        TextureAtlas {
            layout: my_assets.layout.clone(),
            ..default()
        },
        // Specify actor for entity
        DynamicActor {
            actor: ActorMyAssets::Frog.into(),
            animation_timer: Some(Timer::from_seconds(0.25, TimerMode::Repeating)),
            ..default()
        },
    ));
}
