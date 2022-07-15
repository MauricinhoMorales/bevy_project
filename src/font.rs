use bevy::prelude::*;

pub struct MediumFont(pub Handle<Font>);

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

pub struct BoldFont(pub Handle<Font>);

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

pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MediumFont>()
            .init_resource::<BoldFont>();
    }
}
