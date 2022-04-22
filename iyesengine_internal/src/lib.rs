#![allow(unused_variables)]

pub use bevy;
pub use bevy::app as bevy_app;
pub use bevy::asset as bevy_asset;
pub use bevy::core as bevy_core;
pub use bevy::diagnostic as bevy_diagnostic;
pub use bevy::ecs as bevy_ecs;
pub use bevy::input as bevy_input;
pub use bevy::log as bevy_log;
pub use bevy::math as bevy_math;
pub use bevy::reflect as bevy_reflect;
pub use bevy::scene as bevy_scene;
pub use bevy::tasks as bevy_tasks;
pub use bevy::transform as bevy_transform;
pub use bevy::utils as bevy_utils;
pub use bevy::window as bevy_window;

#[cfg(feature = "benimator")]
pub use benimator;

#[cfg(feature = "bevy_asset_loader")]
pub use bevy_asset_loader;

#[cfg(feature = "bevy_asset_ron")]
pub use bevy_asset_ron;

#[cfg(feature = "bevy_ecs_tilemap")]
pub use bevy_ecs_tilemap;

#[cfg(feature = "bevy_kira_audio")]
pub use bevy_kira_audio;

#[cfg(feature = "bevy_prototype_debug_lines")]
pub use bevy_prototype_debug_lines;

#[cfg(feature = "bevy_prototype_lyon")]
pub use bevy_prototype_lyon;

#[cfg(feature = "bevy_tweening")]
pub use bevy_tweening;

#[cfg(feature = "heron")]
pub use heron;

pub mod prelude {
    pub use bevy::prelude::*;

    #[cfg(feature = "bevy_asset_ron")]
    pub use bevy_asset_ron::*;

    #[cfg(feature = "bevy_kira_audio")]
    pub use bevy_kira_audio::{*, Frame as AudioFrame};

    #[cfg(feature = "benimator")]
    pub use benimator::{*, Frame as AnimationFrame};

    #[cfg(feature = "bevy_ecs_tilemap")]
    pub use bevy_ecs_tilemap::prelude::*;

    #[cfg(feature = "bevy_prototype_lyon")]
    pub use bevy_prototype_lyon::prelude::*;

    #[cfg(feature = "heron")]
    pub use heron::prelude::*;

    #[cfg(feature = "bevy_prototype_debug_lines")]
    pub use bevy_prototype_debug_lines::DebugLines;

    #[cfg(not(feature = "bevy_prototype_debug_lines"))]
    pub use crate::DebugLinesDummy as DebugLines;

    pub use crate::IyesExtras;
    pub use crate::IyesEverything;
}

use bevy::prelude::*;

#[derive(Default)]
pub struct DebugLinesDummy;

impl DebugLinesDummy {
    pub fn line(&mut self, start: Vec3, end: Vec3, duration: f32) {}
    pub fn line_colored(&mut self, start: Vec3, end: Vec3, duration: f32, color: Color) {}
    pub fn line_gradient(&mut self, start: Vec3, end: Vec3, duration: f32, start_color: Color, end_color: Color) {}
}

/// Adds and configures all the stuff on top of Bevy
pub struct IyesExtras;

/// Adds `bevy::DefaultPlugins` and `iyesengine::IyesExtras`
pub struct IyesEverything;

impl Plugin for IyesExtras {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "bevy_kira_audio")]
        app.add_plugin(bevy_kira_audio::AudioPlugin::default());
        #[cfg(feature = "bevy_prototype_debug_lines")]
        app.add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin::default());
        #[cfg(feature = "benimator")]
        app.add_plugin(benimator::AnimationPlugin::default());
        #[cfg(feature = "heron")]
        app.add_plugin(heron::PhysicsPlugin::default());
        #[cfg(feature = "bevy_ecs_tilemap")]
        app.add_plugin(bevy_ecs_tilemap::TilemapPlugin::default());
        #[cfg(feature = "bevy_prototype_lyon")]
        app.add_plugin(bevy_prototype_lyon::prelude::ShapePlugin);
    }
}

impl Plugin for IyesEverything {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::DefaultPlugins);
        app.add_plugin(IyesExtras);
    }
}
