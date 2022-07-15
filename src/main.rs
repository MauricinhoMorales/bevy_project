use bevy::prelude::*;

mod asset;
mod button;
mod camara;
mod font;
mod mouse;
mod picking;
mod rotation;
mod text;
mod ui;
mod window;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(window::WindowPlugin)
        // .add_plugin(rotation::ExamplePlugin)
        // .add_plugin(ui::UIPlugin)
        // .add_plugin(font::FontPlugin)
        // .add_plugin(button::ButtonPlugin)
        // .add_plugin(camara::PanOrbitCamaraPlugin)
        // .add_plugin(asset::Scene2Plugin)
        // .add_plugin(mouse::MousePlugin)
        .run();
}
