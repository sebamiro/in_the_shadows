//
// smiro
// 2024-02
// bevy
//

use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Update, print_position);
    }
}
fn print_position(query: Query<(Entity, &Transform)>) {
    for (e, p) in query.iter() {
        info!("Entity {:?} is at position {:?},", e, p.translation);
    }
}
