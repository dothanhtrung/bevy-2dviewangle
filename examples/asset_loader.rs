use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_2dviewangle::{ActorSpriteSheets, View2DAnglePlugin, View2dActor, View2dCollection};
use bevy_asset_loader::prelude::{AssetCollection, ConfigureLoadingState, LoadingState, LoadingStateAppExt};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    InGame,
}

#[derive(AssetCollection, View2dCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "frog_idle_front.png")]
    #[textureview(actor = "frog", action = "idle", angle = "front")]
    pub idle_front: Handle<Image>,

    #[asset(path = "frog_idle_back.png")]
    #[textureview(angle = "back")]
    pub idle_back: Handle<Image>,

    #[asset(path = "frog_idle_left.png")]
    #[textureview(angle = "left")]
    pub idle_left: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 1, rows = 3))]
    #[textureview(angle = "any")]
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
                .continue_to_state(MyStates::InGame)
                .load_collection::<MyAssets>(),
        )
        // Add the plugin
        .add_plugins(View2DAnglePlugin::new(vec![MyStates::InGame]))
        .add_systems(OnEnter(MyStates::InGame), setup)
        .add_systems(Update, input.run_if(in_state(MyStates::InGame)))
        .run();
}

fn setup(mut commands: Commands, mut animation2d: ResMut<ActorSpriteSheets>, my_assets: Res<MyAssets>) {
    // Load into collection
    animation2d.load_asset_loader(my_assets.as_ref());

    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite {
            image: my_assets.idle_front.clone(),
            texture_atlas: Some(TextureAtlas::from(my_assets.front_layout.clone())),
            ..default()
        },
        Transform::from_scale(Vec3::splat(10.)),
        // Specify actor for entity
        View2dActor {
            actor: ActorMyAssets::Frog.into(),
            animation_timer: Some(Timer::from_seconds(0.25, TimerMode::Repeating)),
            ..default()
        },
    ));
}

pub fn input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut actors: Query<(&mut View2dActor, Entity)>,
    mut action_event: EventWriter<ViewChanged>,
) {
    for (mut act, e) in actors.iter_mut() {
        let mut action = act.action;
        let mut direction = act.angle;

        // Update action id and direction of actor
        if kb_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            action = ActionMyAssets::Idle.into();
            direction = Angle::Left;
        } else if kb_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            action = ActionMyAssets::Idle.into();
            direction = Angle::Right;
        } else if kb_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            action = ActionMyAssets::Idle.into();
            direction = Angle::Back;
        } else if kb_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            action = ActionMyAssets::Idle.into();
            direction = Angle::Front;
        }

        if action != act.action || direction != act.angle {
            act.action = action;
            act.angle = direction;
            // Send event to change to sprite sheet to another view
            action_event.write(ViewChanged { entity: e });
        }
    }
}
