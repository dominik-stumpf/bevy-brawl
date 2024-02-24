use crate::{
    ability::{cast_ability::AbilityCast, Ability},
    cursor_caster::CursorPosition,
};
use bevy::{input::keyboard::KeyboardInput, prelude::*};

use super::Player;

pub struct AbilityCasterControllerPlugin;

impl Plugin for AbilityCasterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, initiate_ability_cast);
    }
}

fn initiate_ability_cast(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut cast_event: EventWriter<AbilityCast>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    cursor_position: Res<CursorPosition>,
) {
    let Ok((caster, transform)) = player_query.get_single() else {
        return;
    };

    for event in keyboard_events.read() {
        if event.key_code == KeyCode::KeyQ {
            cast_event.send(AbilityCast {
                ability: Ability::MagicMissile,
                caster,
                cast_origin: transform.translation,
                cast_destination: cursor_position.0,
            });
        }
    }
}
