// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use ambu_snake::AppPlugin;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() -> AppExit {
    App::new()        
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "warn,ui=info".to_string(),
                    level: Level::INFO,
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // You may want this set to `true` if you need virtual keyboard work in mobile browsers.
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(AppPlugin)
        .add_plugins(EguiPlugin)
        .run()
}
