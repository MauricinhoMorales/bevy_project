use bevy::prelude::*;

pub struct MediumFont(pub Handle<Font>);
pub struct BoldFont(pub Handle<Font>);

pub struct MyAssets {
    pub ipfs: Handle<Scene>,
    pub sf: Handle<Scene>
}

// Importing Medium Font
impl FromWorld for MediumFont {
    fn from_world(world: &mut World) -> Self {
        MediumFont(
            world
                .get_resource::<AssetServer>()
                .unwrap()
                .load("fonts/FiraMono-Medium.ttf"),
        )
    }
}

// Importing Bold Font
impl FromWorld for BoldFont {
    fn from_world(world: &mut World) -> Self {
        BoldFont(
            world
                .get_resource::<AssetServer>()
                .unwrap()
                .load("fonts/FiraSans-Bold.ttf"),
        )
    }
}

fn asset_loading(mut commands: Commands, server: Res<AssetServer>){
    commands.insert_resource(
        MyAssets{
            ipfs: server.load("models/Logo_IPFS.gltf#Scene0"),
            sf: server.load("models/Logo_SugarFunge.gltf#Scene0"),
        }
    );
}

pub struct FontPlugin;

//Plugin to initialize different types of fonts
impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MediumFont>()
            .init_resource::<BoldFont>();
    }
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup,asset_loading);
    }
}
