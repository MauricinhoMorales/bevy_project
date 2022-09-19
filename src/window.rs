use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

// Function to show a simple window with a Title and Text
fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world\nViva");
    });
}

pub struct WindowPlugin;

// Plugin to show a simple window inside the main window.
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(ui_example);
    }
}
