//
// smiro
// 2024-02
// bevy
//

use bevy::prelude::*;

pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Update, update_poistion);
    }
}

fn update_poistion(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (v, mut t) in query.iter_mut() {
        t.translation += v.value * time.delta_seconds();
    }
}
