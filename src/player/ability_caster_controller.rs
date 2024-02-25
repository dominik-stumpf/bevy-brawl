use super::Player;
use crate::{
    ability::{cast_ability::AbilityCast, Ability},
    cursor_caster::CursorPosition,
    prelude::*,
};
use bevy::input::keyboard::KeyboardInput;

pub struct AbilityCasterControllerPlugin;

impl Plugin for AbilityCasterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (initiate_ability_cast, tick_recharge_cooldown).run_if(in_state(GameState::InGame)),
        );
    }
}

fn initiate_ability_cast(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut cast_event: EventWriter<AbilityCast>,
    mut play_sfx: EventWriter<audio::EventPlaySFX>,
    mut player_query: Query<(Entity, &Transform, &mut ActiveAbilities), With<Player>>,
    cursor_position: Res<CursorPosition>,
) {
    let Ok((caster, transform, mut abilities)) = player_query.get_single_mut() else {
        return;
    };

    for event in keyboard_events.read() {
        for ability_initiator in abilities.0.iter_mut() {
            if event.key_code == ability_initiator.keyboard_shortcut {
                if ability_initiator.is_recharge_on_cooldown {
                    continue;
                }
                ability_initiator.is_recharge_on_cooldown = true;
                play_sfx.send(audio::EventPlaySFX::new(ability_initiator.cast_sfx));
                cast_event.send(AbilityCast {
                    ability: ability_initiator.ability_type,
                    caster,
                    cast_origin: transform.translation,
                    cast_destination: cursor_position.0,
                });
            }
        }
    }
}

fn tick_recharge_cooldown(
    mut active_abilities_query: Query<&mut ActiveAbilities>,
    time: Res<Time>,
) {
    for mut ability_initiators in active_abilities_query.iter_mut() {
        for initiator in ability_initiators.0.iter_mut() {
            if initiator.is_recharge_on_cooldown {
                initiator.recharge_time.tick(time.delta());
            }
            if initiator.recharge_time.just_finished() {
                initiator.recharge_time.reset();
                initiator.is_recharge_on_cooldown = false;
            }
        }
    }
}

/// Contains data required to initiate ability cast
pub struct AbilityCastInitiator {
    // cast_animation: ...,
    pub cast_time: Timer,
    pub recharge_time: Timer,
    pub cast_sfx: audio::SFXKind,
    is_cast_on_cooldown: bool,
    is_recharge_on_cooldown: bool,
    pub ability_type: Ability,
    pub keyboard_shortcut: KeyCode,
}

impl AbilityCastInitiator {
    pub fn new(
        cast_time: Timer,
        recharge_time: Timer,
        cast_sfx: audio::SFXKind,
        ability_type: Ability,
        keyboard_shortcut: KeyCode,
    ) -> Self {
        Self {
            cast_sfx,
            cast_time,
            recharge_time,
            is_cast_on_cooldown: false,
            is_recharge_on_cooldown: false,
            keyboard_shortcut,
            ability_type,
        }
    }
}

#[derive(Component)]
pub struct ActiveAbilities(pub Vec<AbilityCastInitiator>);
