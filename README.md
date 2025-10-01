<div align="center">

bevy_2dviewangle
================

[![crates.io](https://img.shields.io/crates/v/bevy_2dviewangle)](https://crates.io/crates/bevy_2dviewangle)
[![docs.rs](https://docs.rs/bevy_2dviewangle/badge.svg)](https://docs.rs/bevy_2dviewangle)
[![dependency status](https://deps.rs/repo/gitlab/kimtinh/bevy-2dviewangle/status.svg)](https://deps.rs/repo/gitlab/kimtinh/bevy-2dviewangle)
[![pipeline status](https://gitlab.com/kimtinh/bevy-2dviewangle/badges/master/pipeline.svg)](https://gitlab.com/kimtinh/bevy-2dviewangle/-/commits/master)

[![Gitlab](https://img.shields.io/badge/gitlab-%23181717.svg?style=for-the-badge&logo=gitlab&logoColor=white)](https://gitlab.com/kimtinh/bevy-2dviewangle)
[![Github](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/dothanhtrung/bevy-2dviewangle)

![](examples/2d.mp4)

![](examples/3d.mp4)

</div>

Bevy plugin to easier to manage and switch texture base on view angles.


Quickstart
----------

```rust
// Struct to store sprite sheet
#[derive(View2dCollection, Default)]
struct MyAssets {
    #[textureview(actor = "player", action = "idle", angle = "front")]
    pub idle_front: Handle<Image>,

    // If not specify actor/action, the previous value will be used
    #[textureview(angle = "back")]
    pub idle_back: Handle<Image>,

    // If the angle "right" is not defined, it will be flipped base on the "left" image
    #[textureview(angle = "left")]
    pub idle_left: Handle<Image>,

    // If angle is any, other angle which has not been defined will use this value
    #[textureview(angle = "any")]
    pub idle_any_layout: Handle<TextureAtlasLayout>,
}
```

```rust
// Change the sprite sheet by sending event
fn switch_sprite(
    mut actors: Query<(&mut View2dActor, Entity)>,
    mut action_event: MessageWriter<ViewChanged>,
) {
    for (mut act, e) in actors.iter_mut() {
        act.action = ActionMyAssets::Idle;
        act.angle = Angle::Right;
        action_event.write(ViewChanged { entity: e });
    }
}
```

Please see in [examples](./examples) for more detail.

This plugin can work with [bevy_asset_loader](https://crates.io/crates/bevy_asset_loader) too:

```rust
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
    pub any_layout: Handle<TextureAtlasLayout>,
}
```

License
-------

Please see [LICENSE](./LICENSE).


Compatible Bevy Versions
------------------------

| bevy | bevy_2dviewangle |
|------|------------------|
| 0.16 | 0.10             |
| 0.15 | 0.9              |
| 0.14 | 0.7-0.8          |
| 0.13 | 0.2-0.6          |
| 0.12 | 0.1              |
