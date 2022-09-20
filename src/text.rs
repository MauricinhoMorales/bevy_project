use crate::resources::{BoldFont, MediumFont};
use bevy::prelude::*;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

// Create an example with two different types of text 
fn setup_txt(mut commands: Commands, medium_font: Res<MediumFont>, bold_font: Res<BoldFont>) {
    // Create a TextBundle and added a ColorText type
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {                    
                    bottom: Val::Px(100.0),
                    right: Val::Px(15.0),
                    ..default()},
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "hello\nbevy!",
                TextStyle {
                    font: bold_font.0.clone(),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ).with_alignment(TextAlignment::CENTER),
            ..default()
        })
        .insert(Name::new("Text 1"))
        .insert(ColorText);
    // Create a TextBundle and added a FPSText type
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: medium_font.0.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "60".to_string(),
                        style: TextStyle {
                            font: medium_font.0.clone(),
                            font_size: 20.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Text 2"))
        .insert(FpsText);
}

pub struct TextPlugin;

//Plugin to be able to show the texts in 2D
impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_txt);
    }
}
