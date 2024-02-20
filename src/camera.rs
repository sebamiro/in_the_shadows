//
// smiro
// 2024-02
// bevy
//

use bevy::prelude::*;

const CAMRA_DISTANCE: f32 = 80.;

pub struct CameraPluigin;

impl Plugin for CameraPluigin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., CAMRA_DISTANCE, 0.).looking_at(Vec3::ZERO, Vec3:Z),
        ..Default::default()
    });
}
