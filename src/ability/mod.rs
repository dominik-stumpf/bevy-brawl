use self::{
    cast_ability::CastAbilityPlugin, fireball::FireballPlugin, magic_missile::MagicMissilePlugin,
};
use bevy::prelude::*;

pub mod cast_ability;
mod fireball;
mod magic_missile;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CastAbilityPlugin, MagicMissilePlugin, FireballPlugin));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Ability {
    MagicMissile,
    Fireball,
}
