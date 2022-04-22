# IyesGames Meta-Engine

# [![IyesGames](https://github.com/IyesGames/branding/raw/main/logo/iyesgames.png)](https://iyes.games)

This is my personal distribution of the Bevy Game Engine.

Includes all of my own plugins and helper libraries:
 - [`iyes_bevy_util`](https://github.com/IyesGames/iyes_bevy_util)
 - [`iyes_loopless`](https://github.com/IyesGames/iyes_loopless)
 - [`iyes_progress`](https://github.com/IyesGames/iyes_progress)
 - [`bevy_asset_ron`](https://github.com/IyesGames/bevy_asset_ron)

Includes an assortment of community plugins that I like to
use in my projects:

 - [`bevy_asset_loader`](https://github.com/NiklasEi/bevy_asset_loader)
 - [`bevy_debug_lines`](https://github.com/Toqozz/bevy_debug_lines)
 - [`bevy_tweening`](https://github.com/djeedai/bevy_tweening)
 - [`bevy_kira_audio`](https://github.com/NiklasEi/bevy_kira_audio)
 - [`benimator`](https://github.com/jcornaz/benimator)
 - [`bevy_ecs_tilemap`](https://github.com/StarArawn/bevy_ecs_tilemap)
 - [`bevy_prototype_lyon`](https://github.com/IyesGames/bevy_prototype_lyon)
 - [`heron`](https://github.com/jcornaz/heron)
 - ... more will be added as I need them

(some are optional behind cargo features)

Everything is configured to work well together. When necessary, `iyesengine`
provides glue to make plugins play nice with each other and with my custom
stuff.

All dependencies point at my forks, so I can tweak and reconfigure them as
needed.

## Configuration

`iyesengine` is configured using Cargo features, depending on the game
project. Select the appropriate features to automatically apply my desired
configurations for Bevy and the various plugins.

The available features are:
 - `2d`: configure everything for a 2D game
 - `3d`: configure everything for a 3D game
 - `bevy-ui`: include Bevy UI
 - `audio`: add support for audio with `bevy_kira_audio` (`flac` format support only)
 - `physics`: include `heron`, configured respectively to the `2d` and `3d` features
 - `tilemap`: configure for a 2D tilemap game; implies `2d`
 - `shapes`: add support for 2D shapes (with `bevy_prototype_lyon`)

Additionally, there are `dev` and `release` features, that can be enabled
to apply special configurations for dev and release builds, respectively.

`iyesengine` has a `dynamic` feature that works similar to Bevy's. It will
build a single big dylib with Bevy and all the plugins.

## Using

`Cargo.toml`:
```toml
[dependencies.iyesengine]
git = "https://github.com/IyesGames/iyesengine"
features = [
  # select what you need
  "2d",
  "3d",
  "bevy-ui",
  "audio",
  "physics",
  "tilemap",
  "shapes",
]
```

`main.rs`:
```rust
// use our prelude instead of Bevy!
// it includes bevy+plugins + special stuff
use iyesengine::prelude::*;

fn main() {
    App::new()
        // includes bevy::DefaultPlugins and all the other stuff    
        .add_plugin(IyesEverything)
        // ... add more stuff
        .run();
}
```

This repo offers a `Makefile`, which can help by calling `cargo` with the
correct configuration:
 - `make c`: `cargo check --features iyesengine/dev`
 - `make debug`: `cargo build --features iyesengine/dev`
 - `make run`: `cargo run --features iyesengine/dev,iyesengine/dynamic`
 - `make release`: `cargo build --release --features iyesengine/release`

Copy it to the game's repo to use it.
