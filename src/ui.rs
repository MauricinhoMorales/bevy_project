use bevy::prelude::*;

fn ui_init(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, ui_init);
    }
}
