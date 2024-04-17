bevy_2dviewangle
================

[![crates.io](https://img.shields.io/crates/v/bevy_2dviewangle)](https://crates.io/crates/bevy_2dviewangle)
[![docs.rs](https://docs.rs/bevy_2dviewangle/badge.svg)](https://docs.rs/bevy_2dviewangle)

Bevy plugin to easier to switch texture base on view angles. Currently, support 8 view angles:

* front
* back
* left
* right
* front_left
* front_right
* back_left
* back_right

Quick Start
-----------

```rust
use bevy_2dviewangle::{
    ActorsTextures, ActorsTexturesCollection, Angle, DynamicActor, View2DAnglePlugin,
    ViewChanged,
};

// Each actor may have many actions: idle, walk, run, fight, ...
#[repr(u16)]
enum Action {
    Idle,
}

// Struct to load spritesheet
#[derive(ActorsTexturesCollection, Default)]
struct MyAssets {
    #[textureview(actor = 0, action = 0, angle = "front", handle = "image")]
    pub idle_front: Handle<Image>,

    // If not specify actor/action, the previous value will be used
    #[textureview(angle = "back", handle = "image")]
    pub idle_back: Handle<Image>,

    // If angle is any, other angle which has not been defined will use this value
    #[textureview(angle = "front", handle = "atlas_layout", angle = "any")]
    pub layout: Handle<TextureAtlasLayout>,
}

fn main() {
    App::new()
        ...
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
    let mut my_assets = MyAssets::default();
    my_assets.idle_front = asset_server.load("frog_idle_front.png");
    my_assets.idle_back = asset_server.load("frog_idle_back.png");

    let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 1, 3, None, None);
    my_assets.layout = texture_atlases.add(layout);

    // Load into collection
    animation2d.load_asset_loader(&my_assets);

    commands.spawn((
        SpriteSheetBundle {
            ...
        },
        // Specify actor for entity
        DynamicActor {
            actor: Actor::Frog as u64,
            animation_timer: Some(Timer::from_seconds(0.25, TimerMode::Repeating)),
            ..default()
        },
    ));
    ...
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
        if if kb_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            action = Action::Idle as u16;
            direction = Angle::Back;
        }
        ...

        if action != act.action || direction != act.angle {
            act.action = action;
            act.angle = direction;
            // Send event to change to spritesheet of another view
            action_event.send(ViewChanged { entity: e });
        }
    }
}
```

This plugin can work with [bevy_asset_loader](https://crates.io/crates/bevy_asset_loader) too:

```rust
#[derive(AssetCollection, ActorsTexturesCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "frog_idle_front.png")]
    #[textureview(actor = 0, action = 0, angle = "front", handle = "image")]
    pub idle_front: Handle<Image>,

    #[asset(path = "frog_idle_back.png")]
    #[textureview(angle = "back", handle = "image")]
    pub idle_back: Handle<Image>,

    #[asset(path = "frog_idle_left.png")]
    #[textureview(angle = "left", handle = "image")]
    pub idle_left: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 1, rows = 3))]
    #[textureview(angle = "front", handle = "atlas_layout", angle = "any")]
    pub front_layout: Handle<TextureAtlasLayout>,
}
```

Examples
--------

### 2d

![2d demo](https://i.imgur.com/7fRkkg3.mp4)

[2d example](bevy-2dviewangle/examples/2d.rs)

### 3d

![3d demo](https://i.imgur.com/i3JnW9K.mp4)

[3d example](bevy-2dviewangle/examples/3d.rs)

### Use with bevy_asset_loader

[asset loader example](bevy-2dviewangle/examples/asset_loader.rs)

## License

Please see [LICENSE](./LICENSE).

## Compatible Bevy Versions

| bevy | bevy_2dviewangle         |
|------|--------------------------|
| 0.13 | 0.2-0.3, branch `master` |
| 0.12 | 0.1                      |
