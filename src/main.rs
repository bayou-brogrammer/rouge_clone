// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::module_inception)]

use std::io::Cursor;

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
    winit::{
        cursor::{CursorIcon, CustomCursor},
        WinitWindows,
    },
};
use winit::window::Icon;

use crate::{actors::ActorsPlugin, physics::PhysicsPlugin};

pub mod actors;
#[cfg(feature = "dev")]
pub mod dev;
pub mod physics;

mod app_constants;
pub use self::app_constants::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: AppConstants::APP_NAME.to_string(),
                    resolution: WindowResolution::new(1920.0, 1080.0),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
                file_path: AppConstants::BASE.to_string(),
                meta_check: AssetMetaCheck::Never,
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app.add_systems(Startup, set_window_icon_cursor);

    app.add_plugins(ActorsPlugin);
    #[cfg(feature = "dev")]
    app.add_plugins(crate::dev::DevPlugin);
    app.add_plugins(PhysicsPlugin);

    app.run();
}

fn set_window_icon_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: NonSend<WinitWindows>,
    primary_window: Single<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.into_inner();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!("../assets/icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
    commands
        .entity(primary_entity)
        .insert(CursorIcon::Custom(CustomCursor::Image {
            handle: asset_server.load("cursor.png"),
            hotspot: (5, 5),
        }));
}
