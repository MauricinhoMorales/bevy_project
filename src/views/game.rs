use bevy::prelude::*;

#[derive(Component)]
pub struct GameItem;

use crate::{setup, tools::generics::despawn, Camera2D};

use super::GameState;
use crate::components::{assets, button, camara, cars, text};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(game_setup)
                .with_system(despawn::<Camera2D>),
        )
        .add_system_set(SystemSet::on_update(GameState::Game).with_system(game))
        .add_system_set(
            SystemSet::on_exit(GameState::Game)
                .with_system(despawn::<OnGameScreen>)
                .with_system(setup),
        )
        .add_plugin(button::ButtonPlugin)
        .add_plugin(text::TextPlugin)
        .add_plugin(camara::PanOrbitCamaraPlugin)
        .add_plugin(assets::Scene2Plugin)
        .add_plugin(assets::RotationPlugin)
        .add_plugin(cars::CarPlugin);
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(mut commands: Commands) {
    commands.insert_resource(GameTimer(Timer::from_seconds(50.0, false)));
}

// Tick the timer, and change state when finished
fn game(time: Res<Time>, mut game_state: ResMut<State<GameState>>, mut timer: ResMut<GameTimer>) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu).unwrap();
    }
}
