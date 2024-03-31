
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_asset_loader::prelude::{AssetCollection, ConfigureLoadingState, LoadingState, LoadingStateAppExt};

use bevy_2dviewangle::{
    ActorsTextures, Angle, DynamicActor, View2DAnglePlugin, ViewChanged,
};
use bevy_2dviewangle_derive::ActorsTexturesCollection;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

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

#[derive(AssetCollection, ActorsTexturesCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "frog_idle_front.png")]
    #[textureview(actor = 0, action = 0, angle = "front")]
    pub idle_front: Handle<Image>,

    #[asset(path = "frog_idle_back.png")]
    #[textureview(actor = 0, action = 0, angle = "back")]
    pub idle_back: Handle<Image>,

    #[asset(path = "frog_idle_left.png")]
    #[textureview(actor = 0, action = 0, angle = "left")]
    pub idle_left: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 1, rows = 3))]
    #[textureview(actor = 0, action = 0, angle = "front")]
    pub front_layout: Handle<TextureAtlasLayout>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 1, rows = 3))]
    #[textureview(actor = 0, action = 0, angle = "back")]
    pub back_layout: Handle<TextureAtlasLayout>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 1, rows = 3))]
    #[textureview(actor = 0, action = 0, angle = "left")]
    pub left_layout: Handle<TextureAtlasLayout>,
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut animation2d: ResMut<ActorsTextures>,
    my_assets: ResMut<MyAssets>,
) {
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
