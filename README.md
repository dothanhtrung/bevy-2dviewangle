bevy_2dviewangle
================

[![crates.io](https://img.shields.io/crates/v/bevy_2dviewangle)](https://crates.io/crates/bevy_2dviewangle)
[![docs.rs](https://docs.rs/bevy_2dviewangle/badge.svg)](https://docs.rs/bevy_2dviewangle)
[![dependency status](https://deps.rs/repo/gitlab/kimtinh/bevy-2dviewangle/status.svg)](https://deps.rs/repo/gitlab/kimtinh/bevy-2dviewangle)

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

Add plugin.
```rust
    App::new()
        ...
        .add_plugins(View2DAnglePlugin)
        ...
```

Declare texture map with each actor and action with view angle.
```rust
// Struct to load spritesheet
#[derive(ActorsTexturesCollection, Default)]
struct MyAssets {
    #[textureview(actor = 0, action = 0, angle = "front")]
    pub idle_front: Handle<Image>,

    // If not specify actor/action, the previous value will be used
    #[textureview(angle = "back")]
    pub idle_back: Handle<Image>,

    #[textureview(angle = "front")]
    pub layout: Handle<TextureAtlasLayout>,
    
    // If angle is any, other angle which has not been defined will use this value
    #[textureview(angle = "any")]
    pub layout: Handle<TextureAtlasLayout>,
}
```

Change the sprite sheet by sending event.
```rust
fn switch_sprite(
    mut actors: Query<(&mut DynamicActor, Entity)>,
    mut action_event: EventWriter<ViewChanged>,
) {
    for (mut act, e) in actors.iter_mut() {
        act.action = new_action;
        act.angle = new_direction;
        // Send event to change to sprite sheet to another view
        action_event.send(ViewChanged { entity: e });
    }
}
```

Please see in [examples](./bevy-2dviewangle/examples) for more detail.

This plugin can work with [bevy_asset_loader](https://crates.io/crates/bevy_asset_loader) too:

```rust
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
    #[textureview(angle = "any")]
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
| 0.13 | 0.2-0.4, branch `master` |
| 0.12 | 0.1                      |
