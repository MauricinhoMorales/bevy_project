use bevy::prelude::*;
use crate::{constants::{X_DEFAULT, Y_DEFAULT, Z_DEFAULT, X_SEPARATION, FULL_TURN}, resources::{MyAssets, MyActions}};

#[derive(Component)]
pub struct Movable {
    speed: f32,
    rotation_speed: f32,
    status: bool,
}


//Load an .gltf file to be inserted in the world
pub fn spawn_car(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn_bundle(SceneBundle {
        scene: assets.car.clone(),
        transform: Transform::from_xyz(X_DEFAULT+5.0*X_SEPARATION,Y_DEFAULT,Z_DEFAULT),
        ..Default::default()
    })
    .insert(Name::new("Car"))
    .insert(Movable{
        speed: 5.0,
        rotation_speed: 0.1,
        status: true
    });
}

fn move_items(
    keys: Res<Input<KeyCode>>,
    mut items: Query<(&mut Transform, &Movable)>, 
    time: Res<Time>, 
    actions: Res<MyActions>
) {
    if actions.motion{
        for (mut transform, movable) in items.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);

            for key in keys.get_pressed() {
                if movable.status{
                    match key {
                        KeyCode::W => velocity += forward,
                        KeyCode::S => velocity -= forward,
                        KeyCode::A => {
                            let rotation_change =
                                Quat::from_rotation_y(FULL_TURN * movable.rotation_speed * time.delta_seconds());
                            transform.rotate(rotation_change);
                        }
                        KeyCode::D => {
                            let rotation_change =
                                Quat::from_rotation_y(FULL_TURN * -movable.rotation_speed * time.delta_seconds());
                            transform.rotate(rotation_change);

                        }
                        _ => (),
                    }
                }
            }
            velocity = velocity.normalize_or_zero();
            transform.translation += velocity * time.delta_seconds() * movable.speed
        }
    }
}

pub struct CarPlugin;

//Plugin to create a more complex Scene
impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_car)
            .add_system(move_items);
    }
}