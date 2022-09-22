use bevy::prelude::*;
use crate::{constants::{X_DEFAULT, Y_DEFAULT, Z_DEFAULT, X_SEPARATION, FULL_TURN}, resources::{MyAssets, MyActions}};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Car {
    speed: f32,
    rotation_speed: f32,
    active: bool,
    reload_time: Timer,
    shot_available: bool
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet{
    direction: Vec3,
}


//Load an .gltf file to be inserted in the world
pub fn spawn_car(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn_bundle(SceneBundle {
        scene: assets.car.clone(),
        transform: Transform::from_xyz(X_DEFAULT+5.0*X_SEPARATION,Y_DEFAULT,Z_DEFAULT),
        ..Default::default()
    })
    .insert(Name::new("Car"))
    .insert(Car{
        speed: 5.0,
        rotation_speed: 0.1,
        active: true,
        reload_time: Timer::from_seconds(1.0, true),
        shot_available: true
    });
}

fn car_interactions(
    mut commands: Commands,
    mut items: Query<(&mut Transform, &mut Car)>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>, 
    actions: Res<MyActions>

) {
    if actions.motion{
        for (mut transform, mut car) in items.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);

            for key in keys.get_pressed() {
                if car.active{
                    match key {
                        KeyCode::W => velocity += forward,
                        KeyCode::S => velocity -= forward,
                        KeyCode::A => {
                            let rotation_change =
                                Quat::from_rotation_y(FULL_TURN * car.rotation_speed * time.delta_seconds());
                            transform.rotate(rotation_change);
                        }
                        KeyCode::D => {
                            let rotation_change =
                                Quat::from_rotation_y(FULL_TURN * -car.rotation_speed * time.delta_seconds());
                            transform.rotate(rotation_change);

                        }
                        KeyCode::E => {
                            if car.shot_available{
                                let mut spawn_transform = transform.clone();
                                spawn_transform.translation += Vec3::new(0.0,0.4,0.0);
                                commands
                                    .spawn_bundle(PbrBundle{
                                        mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 0.1, subdivisions: 15 })),
                                        material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
                                        transform: spawn_transform,
                                        ..Default::default()
                                    })
                                    .insert(Name::new("Bullet"))
                                    .insert(Bullet{
                                    direction: local_z
                                    })
                                    .insert(Lifetime {
                                        timer: Timer::from_seconds(2.0, false),
                                    });
                                car.shot_available = false;
                            }
                        }
                        _ => (),
                    }
                    if !car.shot_available{
                        car.reload_time.tick(time.delta());
                        if car.reload_time.just_finished(){
                            car.shot_available = true;
                        }
                    }
                }
            }
            velocity = velocity.normalize_or_zero();
            transform.translation += velocity * time.delta_seconds() * car.speed
        }
    }
}

fn bullet_interactions(
    mut items: Query<(&mut Transform, &Bullet)>, 
    time: Res<Time>, 
    actions: Res<MyActions>
) {
    if actions.motion{
        for (mut transform, bullet) in items.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let forward = -Vec3::new(bullet.direction.x, 0., bullet.direction.z);

            velocity+=forward;
            velocity = velocity.normalize_or_zero();
            transform.translation += velocity * time.delta_seconds() * 10.0
        }
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct CarPlugin;

//Plugin to create a more complex Scene
impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_car)
            .add_system(car_interactions)
            .add_system(bullet_interactions)
            .add_system(bullet_despawn);
    }
}