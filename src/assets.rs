use bevy::prelude::*;
use bevy_text_mesh::prelude::*;
use bevy_mod_picking::{
    DebugEventsPickingPlugin,
    InteractablePickingPlugin,
    PickableBundle,
    PickingPlugin,
    DebugCursorPickingPlugin,
};
use std::f32::consts::PI;
use crate::{constants::{X_DEFAULT, Y_DEFAULT, Z_DEFAULT, Y_SEPARATION, X_SEPARATION}, resources::{MyAssets, MyActions}};
const FULL_TURN: f32 = 2.0 * PI;

// Define a component to designate a rotation speed to an entity.
#[derive(Component)]
struct Rotatable {
    speed: f32,
    status: bool,
}

// Spawn a Minor Light
fn spawn_light(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight{
            intensity: 1500.0,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("Simple Light"));
}

// Spawn a Large Light
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
    })
    .insert(Name::new("Directional Light"));
}

// Spawn a Plain for testing
fn spawn_plain( 
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(X_DEFAULT,Y_DEFAULT,Z_DEFAULT),
            ..Default::default()
        })
        .insert(Name::new("Plain"))
        .insert_bundle(PickableBundle::default());
}

// Spawn a Cube for testing
fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::rgb(0.3, 0.3, 0.5).into()),
            transform: Transform::from_xyz(X_DEFAULT,Y_DEFAULT+Y_SEPARATION,Z_DEFAULT),
            ..Default::default()
        })
        .insert(Name::new("Cube"))
        .insert_bundle(PickableBundle::default());
}

// Create a setup to test the rotation feature
fn spawn_rotatable_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a cube to rotate.
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(X_DEFAULT+4.0*X_SEPARATION,Y_DEFAULT+Y_SEPARATION,Z_DEFAULT),
            ..Default::default()
        })
        .insert(Name::new("Rotatable Cube"))
        .insert_bundle(PickableBundle::default())
        .insert(Rotatable {
            speed: 0.3,
            status: true,
        });
}

// Spawn a 3D Text for testing
fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(TextMeshBundle {
            text_mesh: TextMesh::new_with_color("WELCOME!", font, Color::rgb(1., 1., 0.)),
            transform: Transform::from_xyz(X_DEFAULT+X_SEPARATION,Y_DEFAULT+Y_SEPARATION,Z_DEFAULT),
            ..Default::default()
        })
        .insert(Name::new("Text3D"))
        .insert_bundle(PickableBundle::default());
}

//Load an .gltf file to be inserted in the world
pub fn spawn_gltf(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn_bundle(SceneBundle {
        scene: assets.ipfs.clone(),
        transform: Transform::from_xyz(X_DEFAULT+2.0*X_SEPARATION,Y_DEFAULT+Y_SEPARATION,Z_DEFAULT),
        ..Default::default()
    })
    .insert(Name::new("Icon IPFS"))
    .insert(Rotatable {
        speed: 0.3,
        status: true,
    })
    .insert_bundle(PickableBundle::default());

    commands.spawn_bundle(SceneBundle {
        scene: assets.sf.clone(),
        transform: Transform::from_xyz(X_DEFAULT+3.0*X_SEPARATION,Y_DEFAULT+Y_SEPARATION,Z_DEFAULT),
        ..Default::default()
    })
    .insert(Name::new("Icon Sugarfunge"))
    .insert_bundle(PickableBundle::default());
}

// This system will rotate any entity in the scene with an assigned Rotatable around its z-axis.
fn rotate_items(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>, actions: Res<MyActions>) {
    for (mut transform, cube) in cubes.iter_mut() {
        // The speed is taken as a percentage of a full 360 degree turn.
        // The timers delta_seconds is used to smooth out the movement.
        if actions.rotate{
            if cube.status {
                let rotation_change =
                    Quat::from_rotation_y(FULL_TURN * cube.speed * timer.delta_seconds());
                transform.rotate(rotation_change);
            }
        }
    }
}

pub struct Scene1Plugin;

//Plugin to create a Simple Scene
impl Plugin for Scene1Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_cube)
            .add_startup_system(spawn_light)
            .add_startup_system(spawn_plain);
    }
}

pub struct Scene2Plugin;

//Plugin to create a more complex Scene
impl Plugin for Scene2Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TextMeshPlugin)
            .add_plugin(PickingPlugin)
            // .add_plugin(DebugCursorPickingPlugin) // <- Adds the green debug cursor.
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(DebugEventsPickingPlugin)
            .add_startup_system(spawn_plain)
            .add_startup_system(spawn_directional_light)
            .add_startup_system(spawn_gltf)
            .add_startup_system(spawn_cube)
            .add_startup_system(spawn_text);
    }
}

pub struct RotationPlugin;

//Plugin to display the rotation example
impl Plugin for RotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_rotatable_cube)
            .add_system(rotate_items);
    }
}
