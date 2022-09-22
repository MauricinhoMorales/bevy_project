use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use constants::{HEIGHT,WIDTH};

mod assets;
mod button;
mod camara;
mod resources;
mod picking;
mod text;
mod window;
mod constants;
mod cars;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Testing!".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: true,
            ..default()
        })
        .add_plugin(resources::ResourcesPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(resources::FontPlugin)
        // .add_plugin(window::WindowPlugin)
        .add_plugin(button::ButtonPlugin)
        .add_plugin(text::TextPlugin)
        .add_plugin(camara::PanOrbitCamaraPlugin)
        .add_plugin(assets::Scene2Plugin)
        .add_plugin(assets::RotationPlugin)
        .add_plugin(cars::CarPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
