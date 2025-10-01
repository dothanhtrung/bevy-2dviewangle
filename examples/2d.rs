use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_2dviewangle::{ActorSpriteSheets, Angle, View2DAnglePluginAnyState, View2dActor, ViewChanged};
use bevy_2dviewangle_macro::View2dCollection;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("2D demo"),
                resolution: WindowResolution::new(256, 256),
                ..default()
            }),
            ..default()
        }))
        // Add the plugin
        .add_plugins(View2DAnglePluginAnyState::any())
        .add_systems(Startup, setup)
        .add_systems(Update, input)
        .run();
}

// Struct to load spritesheet
// The derive macro will provide these two enums too:
// `enum ActorMyAssets { Frog }` and `enum ActionMyAssets { Idle }`
#[derive(View2dCollection, Default)]
pub struct MyAssets {
    #[textureview(actor = "frog", action = "idle", angle = "front")]
    pub idle_front: Handle<Image>,

    // If not specify actor/action, the previous value will be used
    #[textureview(angle = "back")]
    pub idle_back: Handle<Image>,

    // If the angle "right" is not defined, it will be flipped base on the angle "left" image
    #[textureview(angle = "left")]
    pub idle_left: Handle<Image>,

    // If angle is any, other angle which has not been defined will use this value
    #[textureview(angle = "any")]
    pub layout: Handle<TextureAtlasLayout>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut animation2d: ResMut<ActorSpriteSheets>,
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

    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            image: my_assets.idle_front.clone(),
            texture_atlas: Some(TextureAtlas::from(my_assets.layout.clone())),
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
    mut action_event: MessageWriter<ViewChanged>,
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
