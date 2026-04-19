//! Sphere-specific weapons for hollow earth world.
//!
//! Unique weapons crafted from materials found in the inverted sphere.

/// Special effects that sphere weapons can apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SphereWeaponEffect {
    /// Refracts and amplifies light.
    LightEffect,
    /// Spreads fungal spores on impact.
    SporeSpread,
    /// Drains energy from the core.
    CoreDrain,
    /// Repels creatures with gravity manipulation.
    GravityRepel,
    /// Illuminates the surrounding area.
    Illuminates,
}

/// Crystal pickaxe for mining and light combat.
#[derive(Debug, Clone)]
pub struct CrystalPickaxe {
    /// Damage dealt per hit.
    pub damage: f32,
    /// Current durability.
    pub durability: f32,
    /// Maximum durability.
    pub max_durability: f32,
    /// Special effect.
    pub effect: SphereWeaponEffect,
}

impl CrystalPickaxe {
    /// Default damage.
    pub const DEFAULT_DAMAGE: f32 = 3.0;
    /// Default durability.
    pub const DEFAULT_DURABILITY: f32 = 200.0;

    /// Create a new crystal pickaxe.
    #[must_use]
    pub fn new() -> Self {
        Self {
            damage: Self::DEFAULT_DAMAGE,
            durability: Self::DEFAULT_DURABILITY,
            max_durability: Self::DEFAULT_DURABILITY,
            effect: SphereWeaponEffect::LightEffect,
        }
    }

    /// Use the weapon, reducing durability.
    ///
    /// Returns true if weapon is still usable.
    pub fn use_weapon(&mut self) -> bool {
        self.durability -= 1.0;
        self.durability > 0.0
    }

    /// Get durability percentage.
    #[must_use]
    pub fn durability_percent(&self) -> f32 {
        self.durability / self.max_durability
    }
}

impl Default for CrystalPickaxe {
    fn default() -> Self {
        Self::new()
    }
}

/// Chitin spear for hunting.
#[derive(Debug, Clone)]
pub struct ChitinSpear {
    /// Damage dealt per hit.
    pub damage: f32,
    /// Attack reach distance.
    pub reach: f32,
    /// Current durability.
    pub durability: f32,
    /// Maximum durability.
    pub max_durability: f32,
}

impl ChitinSpear {
    /// Default damage.
    pub const DEFAULT_DAMAGE: f32 = 5.0;
    /// Default reach.
    pub const DEFAULT_REACH: f32 = 2.0;
    /// Default durability.
    pub const DEFAULT_DURABILITY: f32 = 150.0;

    /// Create a new chitin spear.
    #[must_use]
    pub fn new() -> Self {
        Self {
            damage: Self::DEFAULT_DAMAGE,
            reach: Self::DEFAULT_REACH,
            durability: Self::DEFAULT_DURABILITY,
            max_durability: Self::DEFAULT_DURABILITY,
        }
    }

    /// Use the weapon, reducing durability.
    ///
    /// Returns true if weapon is still usable.
    pub fn use_weapon(&mut self) -> bool {
        self.durability -= 1.0;
        self.durability > 0.0
    }

    /// Get durability percentage.
    #[must_use]
    pub fn durability_percent(&self) -> f32 {
        self.durability / self.max_durability
    }
}

impl Default for ChitinSpear {
    fn default() -> Self {
        Self::new()
    }
}

/// Spore grenade for area denial.
#[derive(Debug, Clone)]
pub struct SporeGrenade {
    /// Damage dealt in the area.
    pub damage: f32,
    /// Effect applied on detonation.
    pub effect: SphereWeaponEffect,
    /// Whether the grenade has been used.
    pub consumed: bool,
}

impl SporeGrenade {
    /// Default damage.
    pub const DEFAULT_DAMAGE: f32 = 4.0;

    /// Create a new spore grenade.
    #[must_use]
    pub fn new() -> Self {
        Self {
            damage: Self::DEFAULT_DAMAGE,
            effect: SphereWeaponEffect::SporeSpread,
            consumed: false,
        }
    }

    /// Use the grenade.
    ///
    /// Returns true if successfully used (single use only).
    pub fn use_weapon(&mut self) -> bool {
        if !self.consumed {
            self.consumed = true;
            true
        } else {
            false
        }
    }

    /// Check if grenade is still usable.
    #[must_use]
    pub fn is_usable(&self) -> bool {
        !self.consumed
    }
}

impl Default for SporeGrenade {
    fn default() -> Self {
        Self::new()
    }
}

/// Core siphon for draining core energy.
#[derive(Debug, Clone)]
pub struct CoreSiphon {
    /// Damage dealt per hit.
    pub damage: f32,
    /// Current durability.
    pub durability: f32,
    /// Maximum durability.
    pub max_durability: f32,
    /// Special effect.
    pub effect: SphereWeaponEffect,
    /// Energy stored from draining.
    pub stored_energy: f32,
}

impl CoreSiphon {
    /// Default damage.
    pub const DEFAULT_DAMAGE: f32 = 8.0;
    /// Default durability.
    pub const DEFAULT_DURABILITY: f32 = 100.0;
    /// Maximum stored energy.
    pub const MAX_STORED_ENERGY: f32 = 50.0;

    /// Create a new core siphon.
    #[must_use]
    pub fn new() -> Self {
        Self {
            damage: Self::DEFAULT_DAMAGE,
            durability: Self::DEFAULT_DURABILITY,
            max_durability: Self::DEFAULT_DURABILITY,
            effect: SphereWeaponEffect::CoreDrain,
            stored_energy: 0.0,
        }
    }

    /// Use the weapon, reducing durability and draining energy.
    ///
    /// Returns energy drained this hit.
    pub fn use_weapon(&mut self) -> f32 {
        self.durability -= 1.0;
        let drain = 5.0;
        self.stored_energy = (self.stored_energy + drain).min(Self::MAX_STORED_ENERGY);
        drain
    }

    /// Consume stored energy.
    pub fn consume_energy(&mut self, amount: f32) -> f32 {
        let consumed = amount.min(self.stored_energy);
        self.stored_energy -= consumed;
        consumed
    }

    /// Get durability percentage.
    #[must_use]
    pub fn durability_percent(&self) -> f32 {
        self.durability / self.max_durability
    }
}

impl Default for CoreSiphon {
    fn default() -> Self {
        Self::new()
    }
}

/// Gravity hammer for endgame combat.
#[derive(Debug, Clone)]
pub struct GravityHammer {
    /// Damage dealt per hit.
    pub damage: f32,
    /// Knockback distance in blocks.
    pub knockback: f32,
    /// Current durability.
    pub durability: f32,
    /// Maximum durability.
    pub max_durability: f32,
    /// Special effect.
    pub effect: SphereWeaponEffect,
}

impl GravityHammer {
    /// Default damage.
    pub const DEFAULT_DAMAGE: f32 = 12.0;
    /// Default knockback distance.
    pub const DEFAULT_KNOCKBACK: f32 = 10.0;
    /// Default durability.
    pub const DEFAULT_DURABILITY: f32 = 80.0;

    /// Create a new gravity hammer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            damage: Self::DEFAULT_DAMAGE,
            knockback: Self::DEFAULT_KNOCKBACK,
            durability: Self::DEFAULT_DURABILITY,
            max_durability: Self::DEFAULT_DURABILITY,
            effect: SphereWeaponEffect::GravityRepel,
        }
    }

    /// Use the weapon, reducing durability.
    ///
    /// Returns true if weapon is still usable.
    pub fn use_weapon(&mut self) -> bool {
        self.durability -= 1.0;
        self.durability > 0.0
    }

    /// Get durability percentage.
    #[must_use]
    pub fn durability_percent(&self) -> f32 {
        self.durability / self.max_durability
    }
}

impl Default for GravityHammer {
    fn default() -> Self {
        Self::new()
    }
}

/// Light staff for utility and illumination.
#[derive(Debug, Clone)]
pub struct LightStaff {
    /// Damage dealt per hit.
    pub damage: f32,
    /// Illumination radius in blocks.
    pub light_radius: f32,
    /// Current durability.
    pub durability: f32,
    /// Maximum durability.
    pub max_durability: f32,
    /// Special effect.
    pub effect: SphereWeaponEffect,
}

impl LightStaff {
    /// Default damage.
    pub const DEFAULT_DAMAGE: f32 = 2.0;
    /// Default light radius.
    pub const DEFAULT_LIGHT_RADIUS: f32 = 16.0;
    /// Default durability.
    pub const DEFAULT_DURABILITY: f32 = 300.0;

    /// Create a new light staff.
    #[must_use]
    pub fn new() -> Self {
        Self {
            damage: Self::DEFAULT_DAMAGE,
            light_radius: Self::DEFAULT_LIGHT_RADIUS,
            durability: Self::DEFAULT_DURABILITY,
            max_durability: Self::DEFAULT_DURABILITY,
            effect: SphereWeaponEffect::Illuminates,
        }
    }

    /// Use the weapon, reducing durability.
    ///
    /// Returns true if weapon is still usable.
    pub fn use_weapon(&mut self) -> bool {
        self.durability -= 1.0;
        self.durability > 0.0
    }

    /// Get durability percentage.
    #[must_use]
    pub fn durability_percent(&self) -> f32 {
        self.durability / self.max_durability
    }
}

impl Default for LightStaff {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_pickaxe_stats() {
        let pickaxe = CrystalPickaxe::new();
        assert!((pickaxe.damage - 3.0).abs() < 0.001);
        assert!((pickaxe.durability - 200.0).abs() < 0.001);
        assert_eq!(pickaxe.effect, SphereWeaponEffect::LightEffect);
    }

    #[test]
    fn test_chitin_spear_stats() {
        let spear = ChitinSpear::new();
        assert!((spear.damage - 5.0).abs() < 0.001);
        assert!((spear.reach - 2.0).abs() < 0.001);
        assert!((spear.durability - 150.0).abs() < 0.001);
    }

    #[test]
    fn test_spore_grenade_single_use() {
        let mut grenade = SporeGrenade::new();
        assert!(grenade.is_usable());
        assert!(grenade.use_weapon());
        assert!(!grenade.is_usable());
        assert!(!grenade.use_weapon()); // Second use fails
    }

    #[test]
    fn test_core_siphon_energy_drain() {
        let mut siphon = CoreSiphon::new();
        let drained = siphon.use_weapon();
        assert!((drained - 5.0).abs() < 0.001);
        assert!((siphon.stored_energy - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_core_siphon_energy_consume() {
        let mut siphon = CoreSiphon::new();
        siphon.stored_energy = 20.0;
        let consumed = siphon.consume_energy(15.0);
        assert!((consumed - 15.0).abs() < 0.001);
        assert!((siphon.stored_energy - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_gravity_hammer_stats() {
        let hammer = GravityHammer::new();
        assert!((hammer.damage - 12.0).abs() < 0.001);
        assert!((hammer.knockback - 10.0).abs() < 0.001);
        assert!((hammer.durability - 80.0).abs() < 0.001);
        assert_eq!(hammer.effect, SphereWeaponEffect::GravityRepel);
    }

    #[test]
    fn test_light_staff_stats() {
        let staff = LightStaff::new();
        assert!((staff.damage - 2.0).abs() < 0.001);
        assert!((staff.light_radius - 16.0).abs() < 0.001);
        assert!((staff.durability - 300.0).abs() < 0.001);
        assert_eq!(staff.effect, SphereWeaponEffect::Illuminates);
    }

    #[test]
    fn test_weapon_durability_usage() {
        let mut pickaxe = CrystalPickaxe::new();
        let initial = pickaxe.durability;
        pickaxe.use_weapon();
        assert!((pickaxe.durability - (initial - 1.0)).abs() < 0.001);
    }

    #[test]
    fn test_weapon_durability_percent() {
        let mut spear = ChitinSpear::new();
        spear.durability = 75.0;
        assert!((spear.durability_percent() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_spore_grenade_stats() {
        let grenade = SporeGrenade::new();
        assert!((grenade.damage - 4.0).abs() < 0.001);
        assert_eq!(grenade.effect, SphereWeaponEffect::SporeSpread);
        assert!(!grenade.consumed);
    }
}
