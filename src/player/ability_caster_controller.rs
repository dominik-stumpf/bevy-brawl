use super::Player;
use crate::{
    ability::{cast_ability::AbilityCast, Ability},
    cursor_caster::CursorPosition,
    prelude::*,
};
use bevy::input::keyboard::KeyboardInput;
use bevy_kira_audio::AudioSource;

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
    mut player_query: Query<
        (
            Entity,
            &Transform,
            &mut ActiveAbilities,
            Option<&RechargeCooldown>,
        ),
        With<Player>,
    >,
    cursor_position: Res<CursorPosition>,
    mut commands: Commands,
) {
    let Ok((caster, transform, abilities, recharge_cooldown)) = player_query.get_single_mut() else {
        return;
    };

    for event in keyboard_events.read() {
        for ability_initiator in abilities.0.iter() {
            if event.key_code == ability_initiator.keyboard_shortcut {
                println!("cast {:?}", ability_initiator.ability_type);
                if let Some(cooldown) = recharge_cooldown {
                    for ability_cooldown in cooldown.0.iter() {
                        if ability_initiator.ability_type == ability_cooldown.ability {
                            println!("on cooldown");
                            return;
                        }
                    }
                }
                commands
                    .entity(caster)
                    .insert(RechargeCooldown(vec![AbilityCooldown {
                        ability: ability_initiator.ability_type,
                        timer: ability_initiator.recharge_time.clone(),
                    }]));
                // ability_initiator.recharge_cooldown = Some(ability_initiator.recharge_time);

                play_sfx.send(audio::EventPlaySFX::new(ability_initiator.cast_sfx.clone()));
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
    mut commands: Commands,
    mut cooldowns: Query<(Entity, &mut RechargeCooldown)>,
    time: Res<Time>,
) {
    for (entity, mut recharge_cooldowns) in &mut cooldowns {
        for ability_cooldown in recharge_cooldowns.0.iter_mut() {
            ability_cooldown.timer.tick(time.delta());

            if ability_cooldown.timer.just_finished() {
                commands.entity(entity).remove::<RechargeCooldown>();
            }
        }
    }
}

/// Contains data required to initiate ability cast
pub struct AbilityCastInitiator {
    // cast_animation: ...,
    pub cast_time: Timer,
    pub recharge_time: Timer,
    pub cast_sfx: Handle<AudioSource>,
    // pub cast_cooldown: Option<Timer>,
    // pub recharge_cooldown: Option<Timer>,
    pub ability_type: Ability,
    pub keyboard_shortcut: KeyCode,
}

struct AbilityCooldown {
    ability: Ability,
    timer: Timer,
}

#[derive(Component)]
struct RechargeCooldown(pub Vec<AbilityCooldown>);

#[derive(Component)]
pub struct ActiveAbilities(pub Vec<AbilityCastInitiator>);
