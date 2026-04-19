//! Sphere-specific equipment for hollow earth survival.
//!
//! Specialized gear designed for navigating the unique challenges
//! of the inverted sphere world.

use super::radiation::ShieldType;

/// Equipment slot types for sphere-specific gear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SphereEquipmentSlot {
    /// Boots that manipulate local gravity.
    GravityBoots,
    /// Shield that protects from core radiation.
    CoreShield,
    /// Device that arrests falls.
    FallArrestor,
    /// Full body suit for core containment.
    ContainmentSuit,
}

/// Gravity boots that allow walking on any surface orientation.
///
/// Uses energy to maintain grip on walls and ceilings.
#[derive(Debug, Clone)]
pub struct GravityBoots {
    /// Energy drain per second when active.
    pub energy_drain: f32,
    /// Current energy level (0.0 to 100.0).
    pub energy: f32,
    /// Whether the boots are currently active.
    pub is_active: bool,
}

impl GravityBoots {
    /// Maximum energy capacity.
    pub const MAX_ENERGY: f32 = 100.0;

    /// Create new gravity boots with full energy.
    #[must_use]
    pub fn new(energy_drain: f32) -> Self {
        Self {
            energy_drain,
            energy: Self::MAX_ENERGY,
            is_active: false,
        }
    }

    /// Activate the gravity boots.
    ///
    /// Returns false if no energy available.
    pub fn activate(&mut self) -> bool {
        if self.energy > 0.0 {
            self.is_active = true;
            true
        } else {
            false
        }
    }

    /// Deactivate the gravity boots.
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Update energy based on delta time.
    ///
    /// Drains energy while active, automatically deactivates when depleted.
    pub fn update(&mut self, delta: f32) {
        if self.is_active {
            self.energy -= self.energy_drain * delta;
            if self.energy <= 0.0 {
                self.energy = 0.0;
                self.is_active = false;
            }
        }
    }

    /// Recharge the boots by a specified amount.
    pub fn recharge(&mut self, amount: f32) {
        self.energy = (self.energy + amount).min(Self::MAX_ENERGY);
    }

    /// Get current energy percentage (0.0 to 1.0).
    #[must_use]
    pub fn energy_percent(&self) -> f32 {
        self.energy / Self::MAX_ENERGY
    }
}

impl Default for GravityBoots {
    fn default() -> Self {
        Self::new(5.0) // Default 5 energy per second drain
    }
}

/// Core shield that provides radiation protection.
#[derive(Debug, Clone)]
pub struct CoreShield {
    /// Type of shield determining protection level.
    pub shield_type: ShieldType,
    /// Current durability (0.0 = broken).
    pub durability: f32,
    /// Maximum durability.
    pub max_durability: f32,
    /// Whether the shield is currently active.
    is_active: bool,
}

impl CoreShield {
    /// Create a new core shield.
    #[must_use]
    pub fn new(shield_type: ShieldType, max_durability: f32) -> Self {
        Self {
            shield_type,
            durability: max_durability,
            max_durability,
            is_active: false,
        }
    }

    /// Activate the shield.
    ///
    /// Returns false if shield is broken.
    pub fn activate(&mut self) -> bool {
        if self.durability > 0.0 {
            self.is_active = true;
            true
        } else {
            false
        }
    }

    /// Deactivate the shield.
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Check if shield is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Apply radiation damage to the shield.
    ///
    /// Returns remaining radiation after shield absorption.
    pub fn absorb_radiation(&mut self, radiation: f32, delta: f32) -> f32 {
        if !self.is_active || self.durability <= 0.0 {
            return radiation;
        }

        let reduction = self.shield_type.reduction_factor();
        let absorbed = radiation * reduction;

        // Shield takes durability damage based on absorbed radiation
        self.durability -= absorbed * delta * 10.0;
        if self.durability <= 0.0 {
            self.durability = 0.0;
            self.is_active = false;
        }

        radiation * (1.0 - reduction)
    }

    /// Repair the shield by a specified amount.
    pub fn repair(&mut self, amount: f32) {
        self.durability = (self.durability + amount).min(self.max_durability);
    }

    /// Get durability percentage (0.0 to 1.0).
    #[must_use]
    pub fn durability_percent(&self) -> f32 {
        self.durability / self.max_durability
    }
}

/// Fall arrestor that prevents fall damage.
///
/// Uses charges that recharge over time.
#[derive(Debug, Clone)]
pub struct FallArrestor {
    /// Current available charges.
    pub charges: u32,
    /// Time since last charge was used.
    pub recharge_time: f32,
    /// Seconds required to recharge one charge.
    pub recharge_rate: f32,
    /// Whether the arrestor is active.
    is_active: bool,
}

impl FallArrestor {
    /// Maximum number of charges.
    pub const MAX_CHARGES: u32 = 3;
    /// Default recharge time per charge in seconds.
    pub const DEFAULT_RECHARGE_TIME: f32 = 60.0;

    /// Create a new fall arrestor with full charges.
    #[must_use]
    pub fn new(recharge_rate: f32) -> Self {
        Self {
            charges: Self::MAX_CHARGES,
            recharge_time: 0.0,
            recharge_rate,
            is_active: false,
        }
    }

    /// Activate the fall arrestor.
    ///
    /// Returns false if no charges available.
    pub fn activate(&mut self) -> bool {
        if self.charges > 0 {
            self.is_active = true;
            true
        } else {
            false
        }
    }

    /// Deactivate the fall arrestor.
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Check if arrestor is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Use a charge to arrest a fall.
    ///
    /// Returns true if a charge was consumed.
    pub fn use_charge(&mut self) -> bool {
        if self.charges > 0 && self.is_active {
            self.charges -= 1;
            self.recharge_time = 0.0;
            if self.charges == 0 {
                self.is_active = false;
            }
            true
        } else {
            false
        }
    }

    /// Update recharge timer.
    pub fn update(&mut self, delta: f32) {
        if self.charges < Self::MAX_CHARGES {
            self.recharge_time += delta;
            if self.recharge_time >= self.recharge_rate {
                self.charges += 1;
                self.recharge_time = 0.0;
            }
        }
    }

    /// Get recharge progress (0.0 to 1.0) for current charge.
    #[must_use]
    pub fn recharge_progress(&self) -> f32 {
        if self.charges >= Self::MAX_CHARGES {
            1.0
        } else {
            self.recharge_time / self.recharge_rate
        }
    }
}

impl Default for FallArrestor {
    fn default() -> Self {
        Self::new(Self::DEFAULT_RECHARGE_TIME)
    }
}

/// Core containment suit for endgame exploration.
///
/// Provides 95% radiation reduction but slows movement by 20%.
/// Requires core fragments to craft.
#[derive(Debug, Clone)]
pub struct CoreContainmentSuit {
    /// Current durability.
    pub durability: f32,
    /// Maximum durability.
    pub max_durability: f32,
    /// Whether the suit is being worn.
    is_active: bool,
}

impl CoreContainmentSuit {
    /// Radiation reduction factor (95%).
    pub const RADIATION_REDUCTION: f32 = 0.95;
    /// Movement speed reduction (20% slower).
    pub const SPEED_PENALTY: f32 = 0.20;

    /// Create a new containment suit.
    #[must_use]
    pub fn new(max_durability: f32) -> Self {
        Self {
            durability: max_durability,
            max_durability,
            is_active: false,
        }
    }

    /// Activate (equip) the suit.
    ///
    /// Returns false if suit is broken.
    pub fn activate(&mut self) -> bool {
        if self.durability > 0.0 {
            self.is_active = true;
            true
        } else {
            false
        }
    }

    /// Deactivate (unequip) the suit.
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Check if suit is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Get the speed multiplier when wearing the suit.
    #[must_use]
    pub fn speed_multiplier(&self) -> f32 {
        if self.is_active {
            1.0 - Self::SPEED_PENALTY
        } else {
            1.0
        }
    }

    /// Calculate effective radiation after suit reduction.
    #[must_use]
    pub fn effective_radiation(&self, base_radiation: f32) -> f32 {
        if self.is_active && self.durability > 0.0 {
            base_radiation * (1.0 - Self::RADIATION_REDUCTION)
        } else {
            base_radiation
        }
    }

    /// Apply radiation damage to the suit.
    pub fn absorb_radiation(&mut self, radiation: f32, delta: f32) {
        if self.is_active && self.durability > 0.0 {
            // Suit degrades based on radiation exposure
            self.durability -= radiation * Self::RADIATION_REDUCTION * delta * 5.0;
            if self.durability <= 0.0 {
                self.durability = 0.0;
                self.is_active = false;
            }
        }
    }

    /// Repair the suit.
    pub fn repair(&mut self, amount: f32) {
        self.durability = (self.durability + amount).min(self.max_durability);
    }

    /// Get durability percentage (0.0 to 1.0).
    #[must_use]
    pub fn durability_percent(&self) -> f32 {
        self.durability / self.max_durability
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_boots_activation() {
        let mut boots = GravityBoots::new(5.0);
        assert!(!boots.is_active);
        assert!(boots.activate());
        assert!(boots.is_active);
        boots.deactivate();
        assert!(!boots.is_active);
    }

    #[test]
    fn test_gravity_boots_energy_drain() {
        let mut boots = GravityBoots::new(10.0);
        boots.activate();
        boots.update(5.0); // 5 seconds at 10 drain = 50 energy used
        assert!((boots.energy - 50.0).abs() < 0.001);
        assert!(boots.is_active);
    }

    #[test]
    fn test_gravity_boots_depleted() {
        let mut boots = GravityBoots::new(50.0);
        boots.activate();
        boots.update(3.0); // 3 seconds at 50 drain = 150, depletes at 100
        assert_eq!(boots.energy, 0.0);
        assert!(!boots.is_active);
    }

    #[test]
    fn test_gravity_boots_recharge() {
        let mut boots = GravityBoots::new(10.0);
        boots.energy = 50.0;
        boots.recharge(30.0);
        assert!((boots.energy - 80.0).abs() < 0.001);
        boots.recharge(50.0); // Should cap at max
        assert!((boots.energy - GravityBoots::MAX_ENERGY).abs() < 0.001);
    }

    #[test]
    fn test_core_shield_radiation_absorption() {
        let mut shield = CoreShield::new(ShieldType::Advanced, 100.0);
        shield.activate();
        let remaining = shield.absorb_radiation(1.0, 1.0);
        // Advanced provides 60% reduction
        assert!((remaining - 0.4).abs() < 0.001);
    }

    #[test]
    fn test_core_shield_durability_damage() {
        let mut shield = CoreShield::new(ShieldType::Basic, 100.0);
        shield.activate();
        shield.absorb_radiation(1.0, 1.0);
        assert!(shield.durability < 100.0);
    }

    #[test]
    fn test_fall_arrestor_charges() {
        let mut arrestor = FallArrestor::new(60.0);
        assert_eq!(arrestor.charges, FallArrestor::MAX_CHARGES);
        arrestor.activate();
        assert!(arrestor.use_charge());
        assert_eq!(arrestor.charges, 2);
    }

    #[test]
    fn test_fall_arrestor_recharge() {
        let mut arrestor = FallArrestor::new(60.0);
        arrestor.activate();
        arrestor.use_charge();
        assert_eq!(arrestor.charges, 2);
        arrestor.update(60.0); // Full recharge time
        assert_eq!(arrestor.charges, 3);
    }

    #[test]
    fn test_containment_suit_radiation_reduction() {
        let mut suit = CoreContainmentSuit::new(100.0);
        suit.activate();
        let effective = suit.effective_radiation(1.0);
        assert!((effective - 0.05).abs() < 0.001); // 95% reduction
    }

    #[test]
    fn test_containment_suit_speed_penalty() {
        let mut suit = CoreContainmentSuit::new(100.0);
        assert!((suit.speed_multiplier() - 1.0).abs() < 0.001);
        suit.activate();
        assert!((suit.speed_multiplier() - 0.8).abs() < 0.001);
    }
}
