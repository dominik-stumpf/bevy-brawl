use bevy::prelude::*;

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

/// Marker components to group entities for cleanup
pub mod cleanup_at {
    use bevy::prelude::*;
    // #[derive(Component)]
    // pub struct MenuClose;
    #[derive(Component)]
    pub struct InGameExit;
}
