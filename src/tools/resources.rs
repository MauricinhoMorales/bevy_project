use bevy::prelude::*;

pub struct MyFonts{
    pub medium: Handle<Font>,
    // pub bold: Handle<Font>
}

pub struct MyAssets {
    pub ipfs: Handle<Scene>,
    pub sf: Handle<Scene>,
    pub car: Handle<Scene>
}

pub struct MyButtons {
    pub play: Handle<Image>,
    pub home: Handle<Image>,
    pub pause: Handle<Image>
}

pub struct MyActions {
    pub rotate: bool,
    pub motion: bool
}

impl FromWorld for MyFonts {
    fn from_world(world: &mut World) -> Self {
        MyFonts{
            medium: world
                .get_resource::<AssetServer>()
                .unwrap()
                .load("fonts/FiraMono-Medium.ttf"),
            // bold: world
            //     .get_resource::<AssetServer>()
            //     .unwrap()
            //     .load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

fn asset_loading(mut commands: Commands, server: Res<AssetServer>){
    commands.insert_resource(
        MyAssets{
            ipfs: server.load("models/Logo_IPFS.gltf#Scene0"),
            sf: server.load("models/Logo_SugarFunge.gltf#Scene0"),
            car: server.load("car-models/race.glb#Scene0")
        }
    );
}

fn button_loading(mut commands: Commands, server: Res<AssetServer>){
    commands.insert_resource(
        MyButtons{
            play: server.load("buttons/Play.png"),
            home: server.load("buttons/Home.png"),
            pause: server.load("buttons/Pause.png"),
        }
    );
}

fn actions_loading(mut commands: Commands){
    commands.insert_resource(
        MyActions{
            rotate: false,
            motion: false
        }
    );
}

pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyFonts>();
    }
}


pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup,asset_loading)
        .add_startup_system_to_stage(StartupStage::PreStartup,button_loading)
        .add_startup_system_to_stage(StartupStage::PreStartup,actions_loading);
    }
}
