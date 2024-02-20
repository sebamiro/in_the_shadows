//
// smiro
// 2024-02
// bevy
//

use bevy::prelude::*;

use crate::movment::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., 0.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.);

#[derive(Bundle)]
pub struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Startup, swpawn_spaceship);
    }
}

fn swpawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpaceshipBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneBundle {
            scene: asset_server.load("Spaceship.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..Default::default()
        },
    });
}
