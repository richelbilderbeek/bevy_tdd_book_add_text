use crate::app::*;
use bevy::prelude::*;
mod app;

fn main() {
    let text = String::from("Hello from main");
    let mut app = create_app(text);
    let add_camera_fn = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, add_camera_fn);
    app.add_plugins(DefaultPlugins);
    app.run();
}
