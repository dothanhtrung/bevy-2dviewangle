bevy_2dviewangle
================

[![crates.io](https://img.shields.io/crates/v/bevy_2dviewangle)](https://crates.io/crates/bevy_2dviewangle)
[![docs.rs](https://docs.rs/bevy_2dviewangle/badge.svg)](https://docs.rs/bevy_2dviewangle)
[![dependency status](https://deps.rs/repo/gitlab/kimtinh/bevy-2dviewangle/status.svg)](https://deps.rs/repo/gitlab/kimtinh/bevy-2dviewangle)

![](https://i.imgur.com/7fRkkg3.mp4)


![](https://i.imgur.com/i3JnW9K.mp4)

Bevy plugin to easier to manage and switch texture base on view angles.


## Quickstart

```rust
// Struct to store sprite sheet
#[derive(ActorsTexturesCollection, Default)]
struct MyAssets {
    #[textureview(actor = "player", action = "idle", angle = "front")]
    pub idle_front: Handle<Image>,

    // If not specify actor/action, the previous value will be used
    #[textureview(angle = "back")]
    pub idle_back: Handle<Image>,

    // If the angle "right" is not defined, it will be flipped base on the angle "left" image
    #[textureview(angle = "left")]
    pub idle_left_layout: Handle<TextureAtlasLayout>,
    
    // If angle is any, other angle which has not been defined will use this value
    #[textureview(angle = "any")]
    pub idle_any_layout: Handle<TextureAtlasLayout>,
}
```

```rust
// Change the sprite sheet by sending event
fn switch_sprite(
    mut actors: Query<(&mut DynamicActor, Entity)>,
    mut action_event: EventWriter<ViewChanged>,
) {
    for (mut act, e) in actors.iter_mut() {
        act.action = Action::Idle;
        act.angle = Angle::Right;
        action_event.send(ViewChanged { entity: e });
    }
}
```

Please see in [examples](./examples) for more detail.

This plugin can work with [bevy_asset_loader](https://crates.io/crates/bevy_asset_loader) too:

```rust
#[derive(AssetCollection, ActorsTexturesCollection, Resource)]
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

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 1, rows = 3))]
    #[textureview(angle = "any")]
    pub any_layout: Handle<TextureAtlasLayout>,
}
```


## License

Please see [LICENSE](./LICENSE).


## Compatible Bevy Versions

| bevy | bevy_2dviewangle         |
|------|--------------------------|
| 0.13 | 0.2-0.5, branch `master` |
| 0.12 | 0.1                      |
