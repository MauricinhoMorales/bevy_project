use bevy::prelude::*;
use bevy_text_mesh::prelude::*;

use bevy_mod_picking::{
    DebugEventsPickingPlugin,
    InteractablePickingPlugin,
    PickableBundle,
    PickingPlugin,
    // DebugCursorPickingPlugin,DefaultPickingPlugins,
};

fn spawn_light(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

fn spawn_directional_light(mut commands: Commands) {
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 4.0 })),
            material: materials.add(Color::rgb(0.3, 0.3, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(12.0, 3.0, 0.0)),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default());
}

fn spawn_plain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default());
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(TextMeshBundle {
            text_mesh: TextMesh::new_with_color("Welcome Home!", font, Color::rgb(1., 1., 0.)),
            transform: Transform::from_xyz(5.0, 3.0, 0.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default());
}

pub fn show_info() {
    info!("Move camera around by using WASD for lateral movement");
    info!("Use Left Shift and Spacebar for vertical movement");
    info!("Use the mouse to look around");
    info!("Press Esc to hide or show the mouse cursor");
}

pub fn spawn_gltf(mut commands: Commands, server: Res<AssetServer>) {
    let ipfs = server.load("models/Logo_IPFS.gltf#Scene0");

    commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(-5.0, 3.0, 0.0),
            global: GlobalTransform::identity(),
        })
        .with_children(|parent| {
            parent.spawn_scene(ipfs);
        });

    let sf = server.load("models/Logo_SugarFunge.gltf#Scene0");
    commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(0.0, 3.0, 0.0),
            global: GlobalTransform::identity(),
        })
        .with_children(|parent| {
            parent.spawn_scene(sf);
        });
}

pub struct Scene1Plugin;

impl Plugin for Scene1Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_cube)
            .add_startup_system(spawn_light);
    }
}

pub struct Scene2Plugin;

impl Plugin for Scene2Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TextMeshPlugin)
            // .add_plugins(DefaultPickingPlugins) // <- Adds Picking, Interaction, and Highlighting plugins.
            // .add_plugin(DebugCursorPickingPlugin) // <- Adds the green debug cursor.
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(DebugEventsPickingPlugin)
            .add_startup_system(spawn_plain)
            .add_startup_system(spawn_directional_light)
            .add_startup_system(spawn_gltf)
            .add_startup_system(spawn_cube)
            .add_startup_system(spawn_text)
            .add_startup_system(show_info);
    }
}
