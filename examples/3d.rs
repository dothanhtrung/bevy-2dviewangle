use bevy::prelude::*;
use bevy::render::camera::Exposure;
use bevy::window::WindowResolution;
use bevy_2dviewangle_macro::View2dCollection;
use bevy_sprite3d::{Sprite3d, Sprite3dPlugin};

use bevy_2dviewangle::{ActorSpriteSheets, Angle, View2DAnglePluginAnyState, View2dActor};

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
        .add_plugins(View2DAnglePluginAnyState::any())
        .add_plugins(Sprite3dPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, load_texture)
        .add_systems(Update, setup.run_if(in_state(GameState::Loading)))
        .add_systems(Update, input.run_if(in_state(GameState::Ready)))
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
    mut next_state: ResMut<NextState<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    commands.spawn((
        PointLight {
            intensity: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4., 10., 4.),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Exposure::INDOOR,
        Transform::from_xyz(0., 2.5, 3.).looking_at(Vec3::Y, Vec3::Y),
    ));

    // plane
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // Spawn frog
    let texture_atlas = TextureAtlas {
        layout: front_handle.layout.as_ref().unwrap().clone(),
        index: 0,
    };
    commands.spawn((
        Sprite3d {
            pixels_per_metre: 8.,
            ..default()
        },
        Sprite {
            image: front_handle.image.as_ref().unwrap().clone(),
            texture_atlas: Some(texture_atlas),
            ..default()
        },
        Transform::from_translation(Vec3::new(0., 0.85, 0.)),
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
