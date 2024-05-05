use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_asset_loader::prelude::{AssetCollection, ConfigureLoadingState, LoadingState, LoadingStateAppExt};

use bevy_2dviewangle::{ActorsTextures, ActorsTexturesCollection, DynamicActor, View2DAnglePlugin};

use crate::common::input;

mod common;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, ActorsTexturesCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "frog_idle_front.png")]
    #[textureview(actor = 0, action = 0, angle = "front")]
    pub idle_front: Handle<Image>,

    #[asset(path = "frog_idle_back.png")]
    #[textureview(angle = "back")]
    pub idle_back: Handle<Image>,

    #[asset(path = "frog_idle_left.png")]
    #[textureview(angle = "left")]
    pub idle_left: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 1, rows = 3))]
    #[textureview(angle = "any", handle = "atlas_layout")]
    pub front_layout: Handle<TextureAtlasLayout>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("2D demo with asset loader"),
                resolution: WindowResolution::new(256., 256.),
                ..default()
            }),
            ..default()
        }))
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<MyAssets>(),
        )
        // Add the plugin
        .add_plugins(View2DAnglePlugin)
        .add_systems(OnEnter(MyStates::Next), setup)
        .add_systems(Update, input.run_if(in_state(MyStates::Next)))
        .run();
}

fn setup(mut commands: Commands, mut animation2d: ResMut<ActorsTextures>, my_assets: Res<MyAssets>) {
    // Load into collection
    animation2d.load_asset_loader(my_assets.as_ref());

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture: my_assets.idle_front.clone(),
            atlas: TextureAtlas {
                layout: my_assets.front_layout.clone(),
                ..default()
            },
            transform: Transform::from_scale(Vec3::splat(10.)),
            ..default()
        },
        // Specify actor for entity
        DynamicActor {
            actor: 0, // actor id
            animation_timer: Some(Timer::from_seconds(0.25, TimerMode::Repeating)),
            ..default()
        },
    ));
}
