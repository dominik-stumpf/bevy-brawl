use super::MainCamera;
use bevy::prelude::*;

pub fn lock_camera_to_entity<T: Component>(
    mut param_query: ParamSet<(
        Query<&Transform, With<T>>,
        Query<(&mut Transform, &MainCamera)>,
    )>,
) {
    let mut target_translation = Vec3::ZERO;
    for target in param_query.p0().iter_mut() {
        target_translation = target.translation;
    }

    for (mut camera_transform, camera) in param_query.p1().iter_mut() {
        camera_transform.translation =
            Transform::from_translation(target_translation + camera.initial_position)
                .looking_at(target_translation, Vec3::Z)
                .translation;
    }
}
