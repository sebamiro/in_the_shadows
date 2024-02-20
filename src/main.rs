//
// smiro
// 2024-02
// bevy
//

use bevy::prelude::*;

mod movment;
mod spaceship;
mod camera;
mod debug;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.15, 0.1, 0)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_systems(Startup, swpawn_spaceship)
        .add_plugins(DefaultPlugins)
        .add_plugins(spaceship::SpaceshipPlugin)
        .add_plugins(camera::CameraPluigin)
        .add_plugins(movment::MovementPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}

