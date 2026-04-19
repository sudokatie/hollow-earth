//! Passive creature AI - simple state machine for wandering behavior.
//!
//! Passive creatures (pigs, cows, sheep, chickens) alternate between
//! idling in place and wandering to nearby locations.
//!
//! Hollow Earth passive creatures have biome restrictions and unique drops.

use glam::Vec3;
use serde::{Deserialize, Serialize};

use super::hostile::BiomeRestriction;

// ============================================================================
// Hollow Earth Passive Creatures
// ============================================================================

/// Types of passive creatures in Hollow Earth.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HollowEarthPassiveType {
    /// Bioluminescent beetle that provides light.
    GlowBeetle,
    /// Herd animal that grazes on moss.
    MossGrazer,
    /// Fish that lives in underground pools.
    CaveFish,
    /// Small creature that recycles organic matter.
    ShellMite,
    /// Flying insect that pollinates fungal growths.
    SporeMoth,
}

/// Drop item from a passive creature.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PassiveDrop {
    /// Glowing gland from GlowBeetle.
    BioluminescentGland,
    /// Hide from MossGrazer.
    Hide,
    /// Meat from MossGrazer or CaveFish.
    Meat,
    /// Shell fragment from ShellMite.
    ShellFragment,
    /// Silk from SporeMoth.
    Silk,
    /// Raw fish from CaveFish.
    RawFish,
}

/// Behavior traits for passive creatures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PassiveBehavior {
    /// Emits light in the environment.
    LightSource,
    /// Travels in herds with others.
    HerdAnimal,
    /// Stays in water.
    WaterDweller,
    /// Breaks down organic matter.
    Recycler,
    /// Helps spread spores/pollen.
    Pollinator,
}

/// Definition of a Hollow Earth passive creature.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HollowEarthPassive {
    /// Creature type.
    pub creature_type: HollowEarthPassiveType,
    /// Health points.
    pub health: u32,
    /// Biome restriction for spawning.
    pub biome_restriction: BiomeRestriction,
    /// Primary drop item.
    pub primary_drop: PassiveDrop,
    /// Secondary drop item (optional).
    pub secondary_drop: Option<PassiveDrop>,
    /// Behavioral traits.
    pub behaviors: Vec<PassiveBehavior>,
    /// Light radius if creature emits light (0 = no light).
    pub light_radius: f32,
    /// Preferred group size for herd behavior.
    pub herd_size: u32,
}

impl HollowEarthPassive {
    /// Create a GlowBeetle - bioluminescent insect that provides light.
    #[must_use]
    pub fn glow_beetle() -> Self {
        Self {
            creature_type: HollowEarthPassiveType::GlowBeetle,
            health: 5,
            biome_restriction: BiomeRestriction::Any,
            primary_drop: PassiveDrop::BioluminescentGland,
            secondary_drop: None,
            behaviors: vec![PassiveBehavior::LightSource],
            light_radius: 8.0,
            herd_size: 1,
        }
    }

    /// Create a MossGrazer - herd animal in moss plains.
    #[must_use]
    pub fn moss_grazer() -> Self {
        Self {
            creature_type: HollowEarthPassiveType::MossGrazer,
            health: 20,
            biome_restriction: BiomeRestriction::MossPlains,
            primary_drop: PassiveDrop::Hide,
            secondary_drop: Some(PassiveDrop::Meat),
            behaviors: vec![PassiveBehavior::HerdAnimal],
            light_radius: 0.0,
            herd_size: 5,
        }
    }

    /// Create a CaveFish - water dweller in deep chasm pools.
    #[must_use]
    pub fn cave_fish() -> Self {
        Self {
            creature_type: HollowEarthPassiveType::CaveFish,
            health: 3,
            biome_restriction: BiomeRestriction::DeepChasm,
            primary_drop: PassiveDrop::RawFish,
            secondary_drop: Some(PassiveDrop::Meat),
            behaviors: vec![PassiveBehavior::WaterDweller],
            light_radius: 0.0,
            herd_size: 8,
        }
    }

    /// Create a ShellMite - recycler creature anywhere on the shell.
    #[must_use]
    pub fn shell_mite() -> Self {
        Self {
            creature_type: HollowEarthPassiveType::ShellMite,
            health: 8,
            biome_restriction: BiomeRestriction::ShellOnly,
            primary_drop: PassiveDrop::ShellFragment,
            secondary_drop: None,
            behaviors: vec![PassiveBehavior::Recycler],
            light_radius: 0.0,
            herd_size: 3,
        }
    }

    /// Create a SporeMoth - pollinator in fungal forests.
    #[must_use]
    pub fn spore_moth() -> Self {
        Self {
            creature_type: HollowEarthPassiveType::SporeMoth,
            health: 4,
            biome_restriction: BiomeRestriction::FungalForest,
            primary_drop: PassiveDrop::Silk,
            secondary_drop: None,
            behaviors: vec![PassiveBehavior::Pollinator, PassiveBehavior::LightSource],
            light_radius: 3.0,
            herd_size: 1,
        }
    }

    /// Check if this creature can spawn in the given biome.
    #[must_use]
    pub fn can_spawn_in_biome(&self, biome: &str) -> bool {
        match self.biome_restriction {
            BiomeRestriction::Any => true,
            BiomeRestriction::ShellOnly => biome == "Shell",
            BiomeRestriction::MossPlains => biome == "MossPlains",
            BiomeRestriction::FungalForest => biome == "FungalForest",
            BiomeRestriction::CrystalCaverns => biome == "CrystalCaverns",
            BiomeRestriction::CoreProximity => biome == "CoreProximity",
            BiomeRestriction::DeepChasm => biome == "DeepChasm",
            BiomeRestriction::MagmaFields => biome == "MagmaFields",
        }
    }

    /// Check if creature has a specific behavior.
    #[must_use]
    pub fn has_behavior(&self, behavior: PassiveBehavior) -> bool {
        self.behaviors.contains(&behavior)
    }

    /// Check if creature emits light.
    #[must_use]
    pub fn emits_light(&self) -> bool {
        self.light_radius > 0.0
    }

    /// Check if creature is a herd animal.
    #[must_use]
    pub fn is_herd_animal(&self) -> bool {
        self.herd_size > 1
    }
}

/// Maximum wander distance from current position.
pub const WANDER_RADIUS: f32 = 10.0;

/// Minimum idle time in seconds.
pub const MIN_IDLE_TIME: f32 = 2.0;

/// Maximum idle time in seconds.
pub const MAX_IDLE_TIME: f32 = 5.0;

/// Distance threshold to consider target reached.
pub const REACH_THRESHOLD: f32 = 0.5;

/// State of a passive creature.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassiveState {
    /// Standing still, waiting.
    Idle,
    /// Moving toward a target position.
    Wandering,
    /// Fleeing from a threat (player attacked).
    Fleeing,
}

impl Default for PassiveState {
    fn default() -> Self {
        Self::Idle
    }
}

/// AI component for passive creatures.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassiveAI {
    /// Current behavior state.
    state: PassiveState,
    /// Target position for wandering/fleeing.
    target: Option<Vec3>,
    /// Timer for current state (counts down).
    timer: f32,
    /// Home position (spawn point).
    home: Vec3,
    /// Flee source position.
    flee_from: Option<Vec3>,
}

impl PassiveAI {
    /// Create a new passive AI at the given home position.
    #[must_use]
    pub fn new(home: Vec3) -> Self {
        Self {
            state: PassiveState::Idle,
            target: None,
            timer: random_idle_time(),
            home,
            flee_from: None,
        }
    }

    /// Get the current state.
    #[must_use]
    pub fn state(&self) -> PassiveState {
        self.state
    }

    /// Get the current target position.
    #[must_use]
    pub fn target(&self) -> Option<Vec3> {
        self.target
    }

    /// Get the home position.
    #[must_use]
    pub fn home(&self) -> Vec3 {
        self.home
    }

    /// Set home position.
    pub fn set_home(&mut self, home: Vec3) {
        self.home = home;
    }

    /// Trigger flee behavior from a threat.
    pub fn flee(&mut self, threat_pos: Vec3, current_pos: Vec3) {
        self.state = PassiveState::Fleeing;
        self.flee_from = Some(threat_pos);

        // Flee in opposite direction
        let away = (current_pos - threat_pos).normalize_or_zero();
        self.target = Some(current_pos + away * WANDER_RADIUS);
        self.timer = 3.0; // Flee for 3 seconds
    }

    /// Update the AI state machine.
    ///
    /// Returns the desired movement direction (normalized or zero).
    pub fn update(&mut self, current_pos: Vec3, dt: f32, rng_value: f32) -> Vec3 {
        self.timer -= dt;

        match self.state {
            PassiveState::Idle => {
                if self.timer <= 0.0 {
                    // Transition to wandering
                    self.state = PassiveState::Wandering;
                    self.target = Some(random_wander_target(current_pos, rng_value));
                    self.timer = 10.0; // Max wander time
                }
                Vec3::ZERO
            }
            PassiveState::Wandering => {
                if let Some(target) = self.target {
                    let to_target = target - current_pos;
                    let dist = to_target.length();

                    if dist < REACH_THRESHOLD || self.timer <= 0.0 {
                        // Reached target or timed out
                        self.state = PassiveState::Idle;
                        self.target = None;
                        self.timer = random_idle_time_seeded(rng_value);
                        Vec3::ZERO
                    } else {
                        // Move toward target (XZ plane only)
                        Vec3::new(to_target.x, 0.0, to_target.z).normalize_or_zero()
                    }
                } else {
                    // No target, go idle
                    self.state = PassiveState::Idle;
                    self.timer = random_idle_time_seeded(rng_value);
                    Vec3::ZERO
                }
            }
            PassiveState::Fleeing => {
                if let Some(target) = self.target {
                    let to_target = target - current_pos;
                    let dist = to_target.length();

                    if dist < REACH_THRESHOLD || self.timer <= 0.0 {
                        // Done fleeing
                        self.state = PassiveState::Idle;
                        self.target = None;
                        self.flee_from = None;
                        self.timer = random_idle_time_seeded(rng_value);
                        Vec3::ZERO
                    } else {
                        Vec3::new(to_target.x, 0.0, to_target.z).normalize_or_zero()
                    }
                } else {
                    self.state = PassiveState::Idle;
                    self.timer = random_idle_time_seeded(rng_value);
                    Vec3::ZERO
                }
            }
        }
    }

    /// Force transition to idle state.
    pub fn force_idle(&mut self) {
        self.state = PassiveState::Idle;
        self.target = None;
        self.timer = random_idle_time();
    }
}

impl Default for PassiveAI {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}

/// Generate a random idle time.
fn random_idle_time() -> f32 {
    // Simple deterministic for testing
    (MIN_IDLE_TIME + MAX_IDLE_TIME) / 2.0
}

/// Generate idle time based on a seed value.
fn random_idle_time_seeded(seed: f32) -> f32 {
    MIN_IDLE_TIME + (seed.abs() % 1.0) * (MAX_IDLE_TIME - MIN_IDLE_TIME)
}

/// Generate a random wander target near current position.
fn random_wander_target(current: Vec3, rng_value: f32) -> Vec3 {
    // Use rng_value to determine angle
    let angle = rng_value * std::f32::consts::TAU;
    let distance = WANDER_RADIUS * 0.5 + (rng_value * 0.5 * WANDER_RADIUS);

    Vec3::new(
        current.x + angle.cos() * distance,
        current.y,
        current.z + angle.sin() * distance,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_passive_ai() {
        let home = Vec3::new(10.0, 64.0, 10.0);
        let ai = PassiveAI::new(home);

        assert_eq!(ai.state(), PassiveState::Idle);
        assert_eq!(ai.home(), home);
        assert!(ai.target().is_none());
    }

    #[test]
    fn test_idle_to_wander_transition() {
        let mut ai = PassiveAI::new(Vec3::ZERO);
        ai.timer = 0.1; // Almost expired

        let pos = Vec3::ZERO;

        // Tick past idle timer
        let _ = ai.update(pos, 0.2, 0.5);

        assert_eq!(ai.state(), PassiveState::Wandering);
        assert!(ai.target().is_some());
    }

    #[test]
    fn test_wander_returns_direction() {
        let mut ai = PassiveAI::new(Vec3::ZERO);
        ai.state = PassiveState::Wandering;
        ai.target = Some(Vec3::new(10.0, 0.0, 0.0));
        ai.timer = 5.0;

        let pos = Vec3::ZERO;
        let dir = ai.update(pos, 0.1, 0.5);

        // Should move toward target (positive X)
        assert!(dir.x > 0.0);
        assert!(dir.length() > 0.0);
    }

    #[test]
    fn test_reach_target_goes_idle() {
        let mut ai = PassiveAI::new(Vec3::ZERO);
        ai.state = PassiveState::Wandering;
        ai.target = Some(Vec3::new(0.1, 0.0, 0.1));
        ai.timer = 5.0;

        let pos = Vec3::ZERO;
        let dir = ai.update(pos, 0.1, 0.5);

        // Should have transitioned to idle (target within threshold)
        assert_eq!(ai.state(), PassiveState::Idle);
        assert_eq!(dir, Vec3::ZERO);
    }

    #[test]
    fn test_flee_behavior() {
        let mut ai = PassiveAI::new(Vec3::ZERO);
        let current = Vec3::new(5.0, 0.0, 5.0);
        let threat = Vec3::new(0.0, 0.0, 0.0);

        ai.flee(threat, current);

        assert_eq!(ai.state(), PassiveState::Fleeing);
        assert!(ai.target().is_some());

        // Target should be away from threat
        let target = ai.target().unwrap();
        let away_dir = (current - threat).normalize();
        let target_dir = (target - current).normalize();
        assert!(away_dir.dot(target_dir) > 0.5); // Roughly same direction
    }

    #[test]
    fn test_flee_ends_after_timer() {
        let mut ai = PassiveAI::new(Vec3::ZERO);
        ai.state = PassiveState::Fleeing;
        ai.target = Some(Vec3::new(100.0, 0.0, 100.0)); // Far away
        ai.timer = 0.1;

        let pos = Vec3::ZERO;

        // Tick past timer
        let _ = ai.update(pos, 0.2, 0.5);

        assert_eq!(ai.state(), PassiveState::Idle);
    }

    #[test]
    fn test_force_idle() {
        let mut ai = PassiveAI::new(Vec3::ZERO);
        ai.state = PassiveState::Wandering;
        ai.target = Some(Vec3::ONE);

        ai.force_idle();

        assert_eq!(ai.state(), PassiveState::Idle);
        assert!(ai.target().is_none());
    }

    #[test]
    fn test_idle_returns_zero_direction() {
        let mut ai = PassiveAI::new(Vec3::ZERO);
        ai.state = PassiveState::Idle;
        ai.timer = 5.0; // Not expired

        let dir = ai.update(Vec3::ZERO, 0.1, 0.5);

        assert_eq!(dir, Vec3::ZERO);
    }

    // ========================================================================
    // Hollow Earth Passive Creature Tests
    // ========================================================================

    #[test]
    fn test_glow_beetle_creation() {
        let beetle = HollowEarthPassive::glow_beetle();
        assert_eq!(beetle.creature_type, HollowEarthPassiveType::GlowBeetle);
        assert_eq!(beetle.primary_drop, PassiveDrop::BioluminescentGland);
        assert!(beetle.emits_light());
        assert_eq!(beetle.light_radius, 8.0);
        assert!(beetle.has_behavior(PassiveBehavior::LightSource));
    }

    #[test]
    fn test_moss_grazer_creation() {
        let grazer = HollowEarthPassive::moss_grazer();
        assert_eq!(grazer.creature_type, HollowEarthPassiveType::MossGrazer);
        assert_eq!(grazer.primary_drop, PassiveDrop::Hide);
        assert_eq!(grazer.secondary_drop, Some(PassiveDrop::Meat));
        assert_eq!(grazer.biome_restriction, BiomeRestriction::MossPlains);
        assert!(grazer.has_behavior(PassiveBehavior::HerdAnimal));
        assert!(grazer.is_herd_animal());
    }

    #[test]
    fn test_cave_fish_creation() {
        let fish = HollowEarthPassive::cave_fish();
        assert_eq!(fish.creature_type, HollowEarthPassiveType::CaveFish);
        assert_eq!(fish.primary_drop, PassiveDrop::RawFish);
        assert_eq!(fish.biome_restriction, BiomeRestriction::DeepChasm);
        assert!(fish.has_behavior(PassiveBehavior::WaterDweller));
    }

    #[test]
    fn test_shell_mite_creation() {
        let mite = HollowEarthPassive::shell_mite();
        assert_eq!(mite.creature_type, HollowEarthPassiveType::ShellMite);
        assert_eq!(mite.primary_drop, PassiveDrop::ShellFragment);
        assert_eq!(mite.biome_restriction, BiomeRestriction::ShellOnly);
        assert!(mite.has_behavior(PassiveBehavior::Recycler));
    }

    #[test]
    fn test_spore_moth_creation() {
        let moth = HollowEarthPassive::spore_moth();
        assert_eq!(moth.creature_type, HollowEarthPassiveType::SporeMoth);
        assert_eq!(moth.primary_drop, PassiveDrop::Silk);
        assert_eq!(moth.biome_restriction, BiomeRestriction::FungalForest);
        assert!(moth.has_behavior(PassiveBehavior::Pollinator));
        assert!(moth.emits_light()); // Slight glow
    }

    #[test]
    fn test_passive_biome_restrictions() {
        let beetle = HollowEarthPassive::glow_beetle();
        assert!(beetle.can_spawn_in_biome("MossPlains")); // Any biome
        assert!(beetle.can_spawn_in_biome("FungalForest"));

        let grazer = HollowEarthPassive::moss_grazer();
        assert!(grazer.can_spawn_in_biome("MossPlains"));
        assert!(!grazer.can_spawn_in_biome("DeepChasm"));
    }

    #[test]
    fn test_passive_drops() {
        assert_eq!(HollowEarthPassive::glow_beetle().primary_drop, PassiveDrop::BioluminescentGland);
        assert_eq!(HollowEarthPassive::moss_grazer().primary_drop, PassiveDrop::Hide);
        assert_eq!(HollowEarthPassive::cave_fish().primary_drop, PassiveDrop::RawFish);
        assert_eq!(HollowEarthPassive::shell_mite().primary_drop, PassiveDrop::ShellFragment);
        assert_eq!(HollowEarthPassive::spore_moth().primary_drop, PassiveDrop::Silk);
    }

    #[test]
    fn test_herd_sizes() {
        assert_eq!(HollowEarthPassive::glow_beetle().herd_size, 1);
        assert!(!HollowEarthPassive::glow_beetle().is_herd_animal());

        assert_eq!(HollowEarthPassive::moss_grazer().herd_size, 5);
        assert!(HollowEarthPassive::moss_grazer().is_herd_animal());

        assert_eq!(HollowEarthPassive::cave_fish().herd_size, 8);
        assert!(HollowEarthPassive::cave_fish().is_herd_animal());
    }
}
