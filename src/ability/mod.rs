use self::{cast_ability::CastAbilityPlugin, magic_missile::MagicMissilePlugin};
use bevy::prelude::*;

pub mod cast_ability;
mod magic_missile;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CastAbilityPlugin, MagicMissilePlugin));
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Ability {
    MagicMissile,
    BallOfFlame,
}
