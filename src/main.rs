use views::{GameState, DisplayQuality, Volume, splash, menu, game};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use tools::constants::{HEIGHT,WIDTH};

mod tools;
mod components;
mod views;

#[derive(Component)]
struct Camera2D;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Testing!".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: true,
            ..default()
        })
        .add_plugin(tools::resources::ResourcesPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(tools::resources::FontPlugin)
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .add_startup_system(setup)
        // Declare the game state, and set its startup value
        .add_state(GameState::Splash)
        // Adds the plugins for each state
        .add_plugin(splash::SplashPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default()).insert(Camera2D);
}