use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub const HEIGHT: f32 = 600.0;
pub const WIDTH: f32 = 1200.0;

mod assets;
mod button;
mod camara;
mod resources;
mod picking;
mod text;
mod window;
mod constants;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Testing!".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: true,
            ..default()
        })
        .add_plugin(resources::AssetPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(resources::FontPlugin)
        // .add_plugin(window::WindowPlugin)
        // .add_plugin(button::ButtonPlugin)
        // .add_plugin(text::TextPlugin)
        .add_plugin(camara::PanOrbitCamaraPlugin)
        .add_plugin(assets::Scene2Plugin)
        .add_plugin(assets::RotationPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
