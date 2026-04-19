//! Core region system for the hollow earth.
//!
//! Defines the dangerous core zones at the center of the sphere
//! and the rare resources found there.

/// Distance from center where radiation starts.
pub const CORE_SAFE_RADIUS: f32 = 3072.0;

/// Distance where danger becomes significant.
pub const CORE_DANGER_RADIUS: f32 = 2048.0;

/// Distance where exposure is lethal.
pub const CORE_LETHAL_RADIUS: f32 = 1024.0;

/// Zones relative to the core.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CoreZone {
    /// Beyond safe radius, no radiation effects.
    Safe,
    /// Approaching core, minor radiation warnings.
    Warning,
    /// Significant radiation damage.
    Danger,
    /// Lethal radiation, rapid death.
    Lethal,
}

/// Resources found in core regions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CoreResource {
    /// Fragments of the core material.
    CoreFragment,
    /// High-energy crystals.
    EnergyCrystal,
    /// Condensed plasma orbs.
    PlasmaOrb,
    /// Mysterious void essence.
    VoidEssence,
}

/// Determine the core zone based on distance from center.
#[must_use]
pub fn core_zone(distance_from_center: f32) -> CoreZone {
    if distance_from_center >= CORE_SAFE_RADIUS {
        CoreZone::Safe
    } else if distance_from_center >= CORE_DANGER_RADIUS {
        CoreZone::Warning
    } else if distance_from_center >= CORE_LETHAL_RADIUS {
        CoreZone::Danger
    } else {
        CoreZone::Lethal
    }
}

/// Get the resources available in a core zone.
///
/// More dangerous zones contain rarer and more valuable resources.
#[must_use]
pub fn core_resources(zone: CoreZone) -> Vec<CoreResource> {
    match zone {
        CoreZone::Safe => vec![],
        CoreZone::Warning => vec![
            CoreResource::CoreFragment,
        ],
        CoreZone::Danger => vec![
            CoreResource::CoreFragment,
            CoreResource::EnergyCrystal,
            CoreResource::PlasmaOrb,
        ],
        CoreZone::Lethal => vec![
            CoreResource::CoreFragment,
            CoreResource::EnergyCrystal,
            CoreResource::PlasmaOrb,
            CoreResource::VoidEssence,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_zone_safe() {
        assert_eq!(core_zone(5000.0), CoreZone::Safe);
        assert_eq!(core_zone(CORE_SAFE_RADIUS), CoreZone::Safe);
    }

    #[test]
    fn test_core_zone_warning() {
        assert_eq!(core_zone(CORE_SAFE_RADIUS - 1.0), CoreZone::Warning);
        assert_eq!(core_zone(CORE_DANGER_RADIUS), CoreZone::Warning);
        assert_eq!(core_zone(2500.0), CoreZone::Warning);
    }

    #[test]
    fn test_core_zone_danger() {
        assert_eq!(core_zone(CORE_DANGER_RADIUS - 1.0), CoreZone::Danger);
        assert_eq!(core_zone(CORE_LETHAL_RADIUS), CoreZone::Danger);
        assert_eq!(core_zone(1500.0), CoreZone::Danger);
    }

    #[test]
    fn test_core_zone_lethal() {
        assert_eq!(core_zone(CORE_LETHAL_RADIUS - 1.0), CoreZone::Lethal);
        assert_eq!(core_zone(500.0), CoreZone::Lethal);
        assert_eq!(core_zone(0.0), CoreZone::Lethal);
    }

    #[test]
    fn test_core_resources_by_zone() {
        // Safe zone has no resources
        assert!(core_resources(CoreZone::Safe).is_empty());

        // Warning has CoreFragment only
        let warning_res = core_resources(CoreZone::Warning);
        assert_eq!(warning_res.len(), 1);
        assert!(warning_res.contains(&CoreResource::CoreFragment));

        // Danger has multiple resources
        let danger_res = core_resources(CoreZone::Danger);
        assert_eq!(danger_res.len(), 3);
        assert!(danger_res.contains(&CoreResource::PlasmaOrb));

        // Lethal has all resources including VoidEssence
        let lethal_res = core_resources(CoreZone::Lethal);
        assert_eq!(lethal_res.len(), 4);
        assert!(lethal_res.contains(&CoreResource::VoidEssence));
    }
}
