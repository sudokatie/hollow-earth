//! Radiation system for hollow sphere world.
//!
//! The core emits deadly radiation that decreases with distance.
//! Players need shields to survive closer to the center.

/// Radiation level from 0.0 (safe) to 1.0 (lethal).
pub type RadiationLevel = f32;

/// Types of radiation shields with different reduction factors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShieldType {
    /// No shield protection.
    None,
    /// Basic radiation suit (30% reduction).
    Basic,
    /// Advanced radiation suit (60% reduction).
    Advanced,
    /// Containment suit (80% reduction).
    Containment,
    /// Ancient artifact protection (95% reduction).
    Artifact,
}

impl ShieldType {
    /// Get the reduction factor for this shield type.
    #[must_use]
    pub fn reduction_factor(self) -> f32 {
        match self {
            ShieldType::None => 0.0,
            ShieldType::Basic => 0.3,
            ShieldType::Advanced => 0.6,
            ShieldType::Containment => 0.8,
            ShieldType::Artifact => 0.95,
        }
    }
}

/// Radiation zones based on radiation level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadiationZone {
    /// Safe zone (radiation < 0.1).
    Safe,
    /// Warning zone (radiation 0.1-0.5).
    Warning,
    /// Danger zone (radiation 0.5-0.9).
    Danger,
    /// Lethal zone (radiation > 0.9).
    Lethal,
}

/// Radiation shield equipment.
#[derive(Debug, Clone, Copy)]
pub struct RadiationShield {
    /// Reduction factor from 0.0 (no protection) to 1.0 (full protection).
    pub reduction_factor: f32,
    /// Type of shield.
    pub shield_type: ShieldType,
}

impl RadiationShield {
    /// Create a new radiation shield.
    #[must_use]
    pub fn new(shield_type: ShieldType) -> Self {
        Self {
            reduction_factor: shield_type.reduction_factor(),
            shield_type,
        }
    }

    /// Create a shield with no protection.
    #[must_use]
    pub fn none() -> Self {
        Self::new(ShieldType::None)
    }
}

impl Default for RadiationShield {
    fn default() -> Self {
        Self::none()
    }
}

/// Calculate radiation level based on distance from sphere center.
///
/// Radiation zones:
/// - Safe (< 0.1): distance > 3072 from center
/// - Warning (0.1-0.5): distance 2048-3072
/// - Danger (0.5-0.9): distance 1024-2048
/// - Lethal (> 0.9): distance < 1024
#[must_use]
pub fn calculate_radiation(distance_from_center: f32) -> RadiationLevel {
    if distance_from_center > 3072.0 {
        // Safe zone
        0.0
    } else if distance_from_center > 2048.0 {
        // Warning zone: linear interpolation from 0.1 to 0.5
        let t = 1.0 - (distance_from_center - 2048.0) / 1024.0;
        0.1 + t * 0.4
    } else if distance_from_center > 1024.0 {
        // Danger zone: linear interpolation from 0.5 to 0.9
        let t = 1.0 - (distance_from_center - 1024.0) / 1024.0;
        0.5 + t * 0.4
    } else {
        // Lethal zone: linear interpolation from 0.9 to 1.0
        let t = 1.0 - (distance_from_center / 1024.0);
        (0.9 + t * 0.1).min(1.0)
    }
}

/// Calculate radiation damage per frame.
///
/// Damage = radiation_level * 10 * delta HP per second.
#[must_use]
pub fn radiation_damage(radiation_level: RadiationLevel, delta: f32) -> f32 {
    radiation_level * 10.0 * delta
}

/// Calculate effective radiation after shield reduction.
///
/// Returns base radiation reduced by shield's reduction factor.
#[must_use]
pub fn effective_radiation(base: RadiationLevel, shield: &RadiationShield) -> RadiationLevel {
    (base * (1.0 - shield.reduction_factor)).clamp(0.0, 1.0)
}

/// Determine the radiation zone for a given radiation level.
#[must_use]
pub fn radiation_zone(level: RadiationLevel) -> RadiationZone {
    if level < 0.1 {
        RadiationZone::Safe
    } else if level < 0.5 {
        RadiationZone::Warning
    } else if level < 0.9 {
        RadiationZone::Danger
    } else {
        RadiationZone::Lethal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_radiation_safe_zone() {
        let radiation = calculate_radiation(3500.0);
        assert!(radiation < 0.1, "Far from center should be safe");
    }

    #[test]
    fn test_calculate_radiation_warning_zone() {
        let radiation = calculate_radiation(2500.0);
        assert!(radiation >= 0.1 && radiation < 0.5, "Mid distance should be warning");
    }

    #[test]
    fn test_calculate_radiation_danger_zone() {
        let radiation = calculate_radiation(1500.0);
        assert!(radiation >= 0.5 && radiation < 0.9, "Close to core should be danger");
    }

    #[test]
    fn test_calculate_radiation_lethal_zone() {
        let radiation = calculate_radiation(500.0);
        assert!(radiation >= 0.9, "Very close to core should be lethal");
    }

    #[test]
    fn test_radiation_damage_calculation() {
        let damage = radiation_damage(0.5, 1.0);
        assert!((damage - 5.0).abs() < 0.001, "0.5 radiation * 10 * 1s = 5 HP");
    }

    #[test]
    fn test_shield_type_reduction_factors() {
        assert!((ShieldType::None.reduction_factor() - 0.0).abs() < 0.001);
        assert!((ShieldType::Basic.reduction_factor() - 0.3).abs() < 0.001);
        assert!((ShieldType::Advanced.reduction_factor() - 0.6).abs() < 0.001);
        assert!((ShieldType::Containment.reduction_factor() - 0.8).abs() < 0.001);
        assert!((ShieldType::Artifact.reduction_factor() - 0.95).abs() < 0.001);
    }

    #[test]
    fn test_effective_radiation_with_shield() {
        let shield = RadiationShield::new(ShieldType::Basic);
        let effective = effective_radiation(1.0, &shield);
        assert!((effective - 0.7).abs() < 0.001, "30% reduction should give 0.7");
    }

    #[test]
    fn test_effective_radiation_artifact_shield() {
        let shield = RadiationShield::new(ShieldType::Artifact);
        let effective = effective_radiation(1.0, &shield);
        assert!((effective - 0.05).abs() < 0.001, "95% reduction should give 0.05");
    }

    #[test]
    fn test_radiation_zone_boundaries() {
        assert_eq!(radiation_zone(0.0), RadiationZone::Safe);
        assert_eq!(radiation_zone(0.09), RadiationZone::Safe);
        assert_eq!(radiation_zone(0.1), RadiationZone::Warning);
        assert_eq!(radiation_zone(0.49), RadiationZone::Warning);
        assert_eq!(radiation_zone(0.5), RadiationZone::Danger);
        assert_eq!(radiation_zone(0.89), RadiationZone::Danger);
        assert_eq!(radiation_zone(0.9), RadiationZone::Lethal);
        assert_eq!(radiation_zone(1.0), RadiationZone::Lethal);
    }

    #[test]
    fn test_radiation_shield_default() {
        let shield = RadiationShield::default();
        assert_eq!(shield.shield_type, ShieldType::None);
        assert!((shield.reduction_factor - 0.0).abs() < 0.001);
    }
}
