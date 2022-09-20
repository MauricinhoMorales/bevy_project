use bevy::{prelude::*, app::PluginGroupBuilder};

pub struct MediumFont(pub Handle<Font>);
pub struct BoldFont(pub Handle<Font>);

pub struct MyAssets {
    pub ipfs: Handle<Scene>,
    pub sf: Handle<Scene>
}

pub struct MyButtons {
    pub play: Handle<Image>,
    pub home: Handle<Image>,
    pub pause: Handle<Image>
}

pub struct MyActions {
    pub rotate: bool
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
        }
    );
}

pub struct MediumFontPlugin;

//Plugin to initialize different types of fonts
impl Plugin for MediumFontPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MediumFont>();
    }
}

pub struct BoldFontPlugin;

impl Plugin for BoldFontPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoldFont>();
    }
}

pub struct FontPlugin;

impl PluginGroup for FontPlugin {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(MediumFontPlugin).add(BoldFontPlugin);
    }
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup,asset_loading)
        .add_startup_system_to_stage(StartupStage::PreStartup,button_loading)
        .add_startup_system_to_stage(StartupStage::PreStartup,actions_loading);
    }
}
