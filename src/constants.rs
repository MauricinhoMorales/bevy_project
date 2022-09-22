use bevy::prelude::Color;
use std::f32::consts::PI;
pub const FULL_TURN: f32 = 2.0 * PI;

//Define the dimensions of the main window
pub const HEIGHT: f32 = 600.0;
pub const WIDTH: f32 = 1200.0;

// Define the specific space between some entities
pub const Y_DEFAULT: f32 = 0.0;
pub const Y_SEPARATION: f32 = 2.0;
pub const X_DEFAULT: f32 = 0.0;
pub const X_SEPARATION: f32 = 5.0;
pub const Z_DEFAULT: f32 = 0.0;
// pub const Z_SEPARATION: f32 = 5.0;

//Define a specific color for each one of the button states
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);