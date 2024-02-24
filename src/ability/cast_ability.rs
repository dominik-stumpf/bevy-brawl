use bevy::prelude::*;

use super::Ability;

pub struct CastAbilityPlugin;

impl Plugin for CastAbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AbilityCast>();
        // .add_systems(Update, cast_ability);
    }
}

/// Event for triggering ability casts
#[derive(Event, Debug)]
pub struct AbilityCast {
    /// Type of ability
    pub ability: Ability,
    /// Point in space from where ability is casted
    pub cast_origin: Vec3,
    /// Point in space to where ability is casted
    pub cast_destination: Vec3,
    /// Caster of the ability
    pub caster: Entity,
}

// fn cast_ability(mut events: EventReader<AbilityCast>) {
//     for event in events.read() {
//         println!("casted ability {:?}", event.ability);
//     }
// }
