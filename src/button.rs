use crate::resources::MediumFont;
use bevy::prelude::*;

//Define a specific color for each one of the button states
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct Panel;

//Function to verify the state of a button and change accordingly
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                info!("Pressed Button");
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                info!("Hovered Button");
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                info!("None Button");
            }
        }
    }
}

fn panel_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Panel>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press Panel".to_string();
                *color = PRESSED_BUTTON.into();
                info!("Pressed Panel");
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover Panel".to_string();
                *color = HOVERED_BUTTON.into();
                info!("Hovered Panel");
            }
            Interaction::None => {
                text.sections[0].value = "Panel".to_string();
                *color = NORMAL_BUTTON.into();
                info!("None Panel");
            }
        }
    }
}

fn setup_button(mut commands: Commands, font: Res<MediumFont>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .insert(Name::new("Button"))
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Button",
                    TextStyle {
                        font: font.0.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    }
                ),
                ..default()
            });
        });
}

fn spawn_panel_2d(mut commands: Commands, font: Res<MediumFont>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::WHITE.into(),
            ..Default::default()
        })
        .insert(Name::new("Panel"))
        .insert(Interaction::None)
        .insert(Panel)
        .with_children(|p| {
            p.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Panel",
                    TextStyle {
                        font: font.0.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                ..default()
            });
        });
}

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_button)
            .add_startup_system(spawn_panel_2d)
            .add_system(panel_system)
            .add_system(button_system);
    }
}
