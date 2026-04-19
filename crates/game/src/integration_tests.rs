//! Integration tests for hollow earth game systems.
//!
//! Tests complex interactions between multiple game systems.

use glam::Vec3;

use crate::building::{create_tether, is_intact, Tether, TetherAnchor};
use crate::crafting::sphere_weapons::{GravityHammer, SporeGrenade, SphereWeaponEffect};
use crate::sphere::{
    disorientation_effects, update_orientation, PlayerOrientation,
    calculate_radiation, effective_radiation, radiation_zone, RadiationShield, RadiationZone,
    ShieldType, CoreContainmentSuit,
};
use crate::survival::{
    AttackCooldown, CombatStats, CoreExposure, CoreExposureStage,
    DamageSource, Health,
};

/// Test gravity orientation and freefall disorientation mechanics.
///
/// Simulates: player at surface, falls off, disorientation increases, lands on opposite side.
#[test]
fn test_gravity_orientation_freefall() {
    // Player starts on surface with stable orientation
    let mut orientation = PlayerOrientation::new(Vec3::Y);
    assert!(orientation.is_on_surface);
    assert!((orientation.disorientation_level - 0.0).abs() < 0.001);

    // Player falls off surface - disorientation increases over time
    // Simulate 5 seconds of freefall (10% per second = 50% disorientation)
    for _ in 0..5 {
        update_orientation(&mut orientation, false, 1.0);
    }
    assert!(!orientation.is_on_surface);
    assert!((orientation.disorientation_level - 0.5).abs() < 0.01);

    // Check disorientation effects at mid-level
    let effects = disorientation_effects(orientation.disorientation_level);
    assert!(effects.screen_wobble > 0.0, "Should have some wobble");
    assert!(!effects.nausea, "Not severe enough for nausea yet");

    // Continue falling - max out disorientation
    for _ in 0..6 {
        update_orientation(&mut orientation, false, 1.0);
    }
    assert!(orientation.disorientation_level >= 0.99);

    // Check severe disorientation effects
    let severe_effects = disorientation_effects(orientation.disorientation_level);
    assert!(severe_effects.nausea, "Should have nausea at max disorientation");
    assert!(severe_effects.input_offset > 0.0, "Should have input offset");

    // Player lands on opposite side - orientation changes, disorientation decreases
    let new_up = Vec3::NEG_Y; // Opposite side of sphere
    orientation.up_vector = new_up;

    // Recovery over 3 seconds (20% per second = 60% recovery)
    for _ in 0..3 {
        update_orientation(&mut orientation, true, 1.0);
    }
    assert!(orientation.is_on_surface);
    assert!(orientation.disorientation_level < 0.5, "Should have recovered significantly");

    // Full recovery after more time
    for _ in 0..3 {
        update_orientation(&mut orientation, true, 1.0);
    }
    assert!((orientation.disorientation_level - 0.0).abs() < 0.01);
}

/// Test radiation shielding and exposure accumulation.
///
/// Simulates: player approaches core, radiation increases, shielding reduces it, exposure accumulates.
#[test]
fn test_radiation_shielding_exposure() {
    // Player starts far from core (safe zone)
    let safe_distance = 3500.0;
    let safe_radiation = calculate_radiation(safe_distance);
    assert!(safe_radiation < 0.1, "Far from core should be safe");
    assert_eq!(radiation_zone(safe_radiation), RadiationZone::Safe);

    // Player approaches core (warning zone)
    let warning_distance = 2500.0;
    let warning_radiation = calculate_radiation(warning_distance);
    assert!(warning_radiation >= 0.1 && warning_radiation < 0.5);
    assert_eq!(radiation_zone(warning_radiation), RadiationZone::Warning);

    // Apply basic shield (30% reduction)
    let basic_shield = RadiationShield::new(ShieldType::Basic);
    let shielded_warning = effective_radiation(warning_radiation, &basic_shield);
    assert!(shielded_warning < warning_radiation, "Shield should reduce radiation");

    // Player enters danger zone
    let danger_distance = 1500.0;
    let danger_radiation = calculate_radiation(danger_distance);
    assert!(danger_radiation >= 0.5 && danger_radiation < 0.9);
    assert_eq!(radiation_zone(danger_radiation), RadiationZone::Danger);

    // Track exposure accumulation
    let mut exposure = CoreExposure::new();
    assert_eq!(exposure.current_stage, CoreExposureStage::Safe);

    // Accumulate exposure over time in danger zone (with advanced shield)
    let advanced_shield = RadiationShield::new(ShieldType::Advanced);
    let shielded_danger = effective_radiation(danger_radiation, &advanced_shield);

    // Simulate 5 seconds of exposure
    exposure.update(shielded_danger, 5.0);
    assert!(exposure.total_exposure > 0.0);

    // Player enters lethal zone without proper protection
    let lethal_distance = 500.0;
    let lethal_radiation = calculate_radiation(lethal_distance);
    assert!(lethal_radiation >= 0.9);
    assert_eq!(radiation_zone(lethal_radiation), RadiationZone::Lethal);

    // With containment suit (80% reduction), still dangerous
    let containment_shield = RadiationShield::new(ShieldType::Containment);
    let shielded_lethal = effective_radiation(lethal_radiation, &containment_shield);
    assert!(shielded_lethal < 0.3, "Containment should reduce lethal to warning levels");

    // Extended exposure eventually reaches critical
    for _ in 0..10 {
        exposure.update(shielded_lethal, 1.0);
    }

    // Check sickness effects
    let effects = exposure.effects();
    assert!(effects.speed_penalty > 0.0, "Should have movement penalty");
}

/// Test tether freefall rescue mechanics.
///
/// Simulates: player tethered to anchor, falls, tether catches them.
#[test]
fn test_tether_freefall_rescue() {
    // Create anchor point on surface
    let anchor_pos = Vec3::new(100.0, 0.0, 100.0);
    let anchor = TetherAnchor::new(anchor_pos);
    assert!(anchor.attached_tethers.is_empty());

    // Player position before fall
    let player_start = Vec3::new(110.0, 0.0, 100.0);

    // Create tether between anchor and player
    let tether = create_tether(anchor_pos, player_start);
    assert!(tether.is_ok(), "Should create valid tether within range");
    let tether = tether.unwrap();
    assert!((tether.length - 10.0).abs() < 0.001);

    // Verify tether is intact
    assert!(is_intact(&tether), "New tether should be intact");
    assert!(!tether.broken);
    assert_eq!(tether.current_tension, 0.0);

    // Simulate player falling and tether extending
    // Tether max length is 32 blocks, player started 10 blocks away
    let max_fall_distance = 32.0 - 10.0; // 22 blocks before tether pulls taut

    // Player falls but stays within tether range
    let player_falling = Vec3::new(110.0, -15.0, 100.0);
    let fall_distance = anchor_pos.distance(player_falling);
    assert!(fall_distance < 32.0, "Player should still be within tether range");

    // Create a new tether to player's falling position (simulating dynamic tether)
    let rescue_tether = create_tether(anchor_pos, player_falling);
    assert!(rescue_tether.is_ok(), "Rescue tether should be valid");

    // Player at max tether range
    let player_max_fall = Vec3::new(anchor_pos.x + 30.0, 0.0, anchor_pos.z);
    let max_tether = create_tether(anchor_pos, player_max_fall);
    assert!(max_tether.is_ok(), "Should allow tether at max range");

    // Player beyond tether range
    let player_too_far = Vec3::new(anchor_pos.x + 50.0, 0.0, anchor_pos.z);
    let too_far_tether = create_tether(anchor_pos, player_too_far);
    assert!(too_far_tether.is_err(), "Should fail beyond max range");
}

/// Test weapon effects in combat.
///
/// Simulates: gravity hammer repels creature, spore grenade AOE effects.
#[test]
fn test_weapon_effects_combat() {
    // Gravity hammer setup
    let mut hammer = GravityHammer::new();
    assert_eq!(hammer.effect, SphereWeaponEffect::GravityRepel);
    assert!((hammer.damage - 12.0).abs() < 0.001);
    assert!((hammer.knockback - 10.0).abs() < 0.001);

    // Simulate hitting a creature
    let mut creature_health = Health::new(30.0);
    let before_health = creature_health.current();

    // Apply hammer damage
    creature_health.damage(hammer.damage, DamageSource::Attack);
    hammer.use_weapon();

    assert_eq!(creature_health.current(), before_health - hammer.damage);
    assert!(hammer.durability < GravityHammer::DEFAULT_DURABILITY);

    // Verify knockback would be applied (direction * knockback distance)
    let attacker_pos = Vec3::ZERO;
    let creature_pos = Vec3::new(5.0, 0.0, 0.0);
    let knockback_direction = (creature_pos - attacker_pos).normalize();
    let knockback_velocity = knockback_direction * hammer.knockback;
    assert!(knockback_velocity.length() > 5.0, "Gravity hammer should have strong knockback");

    // Spore grenade setup
    let mut grenade = SporeGrenade::new();
    assert_eq!(grenade.effect, SphereWeaponEffect::SporeSpread);
    assert!(grenade.is_usable());

    // Simulate AOE damage to multiple creatures
    let aoe_radius = 5.0;
    let grenade_pos = Vec3::new(10.0, 0.0, 10.0);
    let creature_positions = vec![
        Vec3::new(12.0, 0.0, 10.0), // 2 units away - hit
        Vec3::new(10.0, 0.0, 13.0), // 3 units away - hit
        Vec3::new(20.0, 0.0, 10.0), // 10 units away - miss
    ];

    let mut hits = 0;
    for creature_pos in &creature_positions {
        if creature_pos.distance(grenade_pos) <= aoe_radius {
            hits += 1;
        }
    }
    assert_eq!(hits, 2, "Should hit 2 creatures in AOE radius");

    // Use grenade (single use)
    assert!(grenade.use_weapon());
    assert!(!grenade.is_usable());
    assert!(!grenade.use_weapon()); // Can't use again
}

/// Test full survival scenario in fungal forest.
///
/// Simulates: player in fungal forest, gets cold, builds shelter, hunts creatures.
#[test]
fn test_full_survival_scenario() {
    // Player starts in fungal forest biome with full health
    let mut player_health = Health::new(20.0);
    assert!(player_health.is_full());

    // Environmental temperature affects player (simulated cold damage over time)
    let cold_damage_per_second = 0.5;
    let exposure_time = 4.0;
    let total_cold_damage = cold_damage_per_second * exposure_time;

    player_health.damage(total_cold_damage, DamageSource::Environment);
    assert_eq!(player_health.current(), 20.0 - total_cold_damage);

    // Player builds shelter (stops environmental damage)
    let has_shelter = true;

    // Player hunts a creature for food
    let creature_health_value = 10.0; // Pig-like creature
    let mut creature_health = Health::new(creature_health_value);

    // Player attacks with chitin spear (5 damage)
    let spear_damage = 5.0;
    creature_health.damage(spear_damage, DamageSource::Attack);
    assert!(!creature_health.is_dead());

    // Clear invincibility for test
    creature_health.tick(1.0);

    // Second attack kills
    let died = creature_health.damage(spear_damage, DamageSource::Attack);
    assert!(died);
    assert!(creature_health.is_dead());

    // Player eats food from creature, heals
    let health_before_heal = player_health.current();
    let food_heal = 3.0;
    let healed = player_health.heal(food_heal);
    // Healed amount should be positive and player health should increase
    assert!(healed > 0.0, "Should heal some amount");
    assert!(player_health.current() > health_before_heal, "Health should increase");

    // In shelter, player regenerates fully over time
    if has_shelter {
        player_health.restore();
        assert!(player_health.is_full());
    }
}

/// Test core approach with containment suit.
///
/// Simulates: player with containment suit approaches core, radiation stages progress.
#[test]
fn test_core_approach() {
    let mut exposure = CoreExposure::new();
    assert_eq!(exposure.current_stage, CoreExposureStage::Safe);

    // Player has containment suit (80% radiation reduction)
    let containment_suit = RadiationShield::new(ShieldType::Containment);
    assert!((containment_suit.reduction_factor - 0.8).abs() < 0.001);

    // Approach through different zones, tracking exposure stages

    // Warning zone (2500 units from center)
    let warning_radiation = calculate_radiation(2500.0);
    let shielded_warning = effective_radiation(warning_radiation, &containment_suit);
    exposure.update(shielded_warning, 2.0);
    // Still safe due to suit protection
    assert!(exposure.current_stage == CoreExposureStage::Safe
        || exposure.current_stage == CoreExposureStage::Caution);

    // Danger zone (1500 units from center)
    let danger_radiation = calculate_radiation(1500.0);
    let shielded_danger = effective_radiation(danger_radiation, &containment_suit);
    exposure.update(shielded_danger, 5.0);
    // Exposure building up
    assert!(exposure.total_exposure > 0.1);

    // Lethal zone (500 units from center)
    let lethal_radiation = calculate_radiation(500.0);
    let shielded_lethal = effective_radiation(lethal_radiation, &containment_suit);
    assert!(shielded_lethal < 0.3, "Suit should reduce lethal to manageable levels");

    // Extended exposure in lethal zone
    for _ in 0..10 {
        exposure.update(shielded_lethal, 1.0);
    }

    // Check progressive sickness effects
    let effects = exposure.effects();
    if exposure.total_exposure > 0.5 {
        assert!(effects.speed_penalty > 0.0, "Should have movement penalty at high exposure");
    }

    // Retreat to safe zone and recover
    let safe_radiation = calculate_radiation(3500.0);
    assert!(safe_radiation < 0.1);

    // Save exposure before recovery
    let exposure_before_recovery = exposure.total_exposure;

    // Recovery in safe zone
    for _ in 0..20 {
        exposure.recover(1.0, true);
    }
    assert!(exposure.total_exposure < exposure_before_recovery || exposure.total_exposure == 0.0,
        "Exposure should decrease in safe zone");
}

/// Test multiple interacting systems in survival scenario.
#[test]
fn test_combined_systems_stress() {
    // Player with various equipment
    let mut player_health = Health::new(20.0);
    let mut orientation = PlayerOrientation::new(Vec3::Y);
    let mut exposure = CoreExposure::new();
    let shield = RadiationShield::new(ShieldType::Advanced);

    // Simulate complex scenario: player near core, in freefall, taking radiation

    // Step 1: Player in danger zone
    let danger_radiation = calculate_radiation(1500.0);
    let shielded_radiation = effective_radiation(danger_radiation, &shield);

    // Step 2: Player falls off surface
    update_orientation(&mut orientation, false, 1.0);
    assert!(!orientation.is_on_surface);

    // Step 3: Radiation exposure while falling
    exposure.update(shielded_radiation, 1.0);

    // Step 4: Disorientation effects compound with radiation sickness
    let disorientation = disorientation_effects(orientation.disorientation_level);
    let sickness = exposure.effects();

    // Combined effects should be non-zero
    let combined_penalty = disorientation.screen_wobble + sickness.speed_penalty;
    // Initially may be zero, but after more time...

    // Step 5: Extended scenario
    for _ in 0..5 {
        update_orientation(&mut orientation, false, 1.0);
        exposure.update(shielded_radiation, 1.0);
    }

    // Now check compounded effects
    let final_disorientation = disorientation_effects(orientation.disorientation_level);
    let final_sickness = exposure.effects();

    assert!(
        final_disorientation.screen_wobble > 0.0 || final_sickness.speed_penalty > 0.0,
        "Should have accumulated negative effects"
    );

    // Step 6: Recovery - land on surface and retreat
    for _ in 0..5 {
        update_orientation(&mut orientation, true, 1.0);
        exposure.recover(1.0, false);
    }

    assert!(orientation.disorientation_level < 0.5, "Should recover from disorientation");
}

/// Test combat stats and cooldown mechanics.
#[test]
fn test_combat_cooldown_mechanics() {
    let stats = CombatStats::default()
        .with_damage(5.0)
        .with_cooldown(1.0)
        .with_reach(4.0);

    let mut cooldown = AttackCooldown::new();
    assert!(cooldown.is_ready());

    // Start attack
    cooldown.start(stats.attack_cooldown);
    assert!(!cooldown.is_ready());

    // Partial cooldown
    cooldown.tick(0.5);
    assert!(!cooldown.is_ready());
    assert!((cooldown.progress() - 0.5).abs() < 0.01);

    // Full cooldown
    cooldown.tick(0.6);
    assert!(cooldown.is_ready());
}
