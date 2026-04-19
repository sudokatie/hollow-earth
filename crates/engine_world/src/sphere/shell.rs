//! Shell interior layer system.
//!
//! Defines the layers within the shell from surface to bedrock,
//! including vacuum breach detection and mineral distribution.

use noise::{NoiseFn, Perlin};

/// Layers within the shell, from surface to bedrock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShellLayer {
    /// The inner surface (depth 0).
    Surface,
    /// Shallow layer (depth 1-16).
    Shallow,
    /// Mid layer (depth 17-32).
    Mid,
    /// Deep layer (depth 33-48).
    Deep,
    /// Bedrock layer near void (depth 49-64).
    Bedrock,
}

/// Properties of a shell layer.
#[derive(Clone, Copy, Debug)]
pub struct ShellProperties {
    /// Rock density (affects mining speed).
    pub density: f32,
    /// Mineral content multiplier.
    pub mineral_content: f32,
    /// Structural stability (affects cave formation).
    pub stability: f32,
    /// Whether this location has a vacuum breach.
    pub has_vacuum_breach: bool,
}

impl ShellProperties {
    /// Create new shell properties.
    #[must_use]
    pub const fn new(density: f32, mineral_content: f32, stability: f32, has_vacuum_breach: bool) -> Self {
        Self {
            density,
            mineral_content,
            stability,
            has_vacuum_breach,
        }
    }
}

/// Types of minerals that can be found in the shell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MineralType {
    /// Common iron ore.
    Iron,
    /// Copper ore for electrical components.
    Copper,
    /// Glowing crystals for light sources.
    Crystal,
    /// Rare fragments from the core.
    CoreFragment,
    /// Powerful energy crystals.
    EnergyCrystal,
}

/// Determine the shell layer at a given depth.
///
/// Depth 0 is the surface, increasing toward bedrock.
#[must_use]
pub fn shell_layer(depth: i32) -> ShellLayer {
    match depth {
        0 => ShellLayer::Surface,
        1..=16 => ShellLayer::Shallow,
        17..=32 => ShellLayer::Mid,
        33..=48 => ShellLayer::Deep,
        _ => ShellLayer::Bedrock,
    }
}

/// Check if a vacuum breach exists at this location.
///
/// Vacuum breaches only occur in Deep and Bedrock layers.
/// They are rare pockets of void that cause instant death.
#[must_use]
pub fn has_vacuum_breach(layer: ShellLayer, noise_val: f32) -> bool {
    match layer {
        ShellLayer::Surface | ShellLayer::Shallow | ShellLayer::Mid => false,
        ShellLayer::Deep => noise_val > 0.92,
        ShellLayer::Bedrock => noise_val > 0.85,
    }
}

/// Determine what mineral exists at a location, if any.
///
/// Mineral distribution depends on depth and noise values.
#[must_use]
pub fn mineral_at_depth(depth: i32, lat: f32, lon: f32, seed: u64) -> Option<MineralType> {
    let layer = shell_layer(depth);

    // Create noise for mineral distribution
    let mineral_noise = Perlin::new(seed.wrapping_mul(13579) as u32);
    let rarity_noise = Perlin::new(seed.wrapping_mul(24680) as u32);
    let offset = ((seed as f64) * 3141.0) % 100_000.0;

    let scale = 0.2;
    let x = (lat as f64) * scale + offset;
    let y = (lon as f64) * scale + offset;
    let z = (depth as f64) * 0.1 + offset;

    let mineral_val = mineral_noise.get([x, y, z]);
    let rarity_val = rarity_noise.get([x * 2.0, y * 2.0, z * 2.0]);

    // Most blocks have no minerals
    if mineral_val < 0.5 {
        return None;
    }

    // Determine mineral type based on layer and noise
    match layer {
        ShellLayer::Surface => None, // No minerals on surface
        ShellLayer::Shallow => {
            if mineral_val > 0.75 {
                Some(MineralType::Iron)
            } else if mineral_val > 0.65 {
                Some(MineralType::Copper)
            } else {
                None
            }
        }
        ShellLayer::Mid => {
            if mineral_val > 0.85 && rarity_val > 0.7 {
                Some(MineralType::Crystal)
            } else if mineral_val > 0.7 {
                Some(MineralType::Copper)
            } else if mineral_val > 0.55 {
                Some(MineralType::Iron)
            } else {
                None
            }
        }
        ShellLayer::Deep => {
            if mineral_val > 0.9 && rarity_val > 0.85 {
                Some(MineralType::EnergyCrystal)
            } else if mineral_val > 0.8 {
                Some(MineralType::Crystal)
            } else if mineral_val > 0.6 {
                Some(MineralType::Copper)
            } else {
                None
            }
        }
        ShellLayer::Bedrock => {
            if mineral_val > 0.95 && rarity_val > 0.9 {
                Some(MineralType::CoreFragment)
            } else if mineral_val > 0.85 && rarity_val > 0.75 {
                Some(MineralType::EnergyCrystal)
            } else if mineral_val > 0.7 {
                Some(MineralType::Crystal)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_layer_surface() {
        assert_eq!(shell_layer(0), ShellLayer::Surface);
    }

    #[test]
    fn test_shell_layer_boundaries() {
        assert_eq!(shell_layer(1), ShellLayer::Shallow);
        assert_eq!(shell_layer(16), ShellLayer::Shallow);
        assert_eq!(shell_layer(17), ShellLayer::Mid);
        assert_eq!(shell_layer(32), ShellLayer::Mid);
        assert_eq!(shell_layer(33), ShellLayer::Deep);
        assert_eq!(shell_layer(48), ShellLayer::Deep);
        assert_eq!(shell_layer(49), ShellLayer::Bedrock);
        assert_eq!(shell_layer(64), ShellLayer::Bedrock);
    }

    #[test]
    fn test_vacuum_breach_safe_layers() {
        // Surface, Shallow, and Mid never have vacuum breaches
        assert!(!has_vacuum_breach(ShellLayer::Surface, 0.99));
        assert!(!has_vacuum_breach(ShellLayer::Shallow, 0.99));
        assert!(!has_vacuum_breach(ShellLayer::Mid, 0.99));
    }

    #[test]
    fn test_vacuum_breach_deep_layer() {
        // Deep layer: breach at noise > 0.92
        assert!(!has_vacuum_breach(ShellLayer::Deep, 0.91));
        assert!(has_vacuum_breach(ShellLayer::Deep, 0.95));
    }

    #[test]
    fn test_vacuum_breach_bedrock_layer() {
        // Bedrock layer: breach at noise > 0.85
        assert!(!has_vacuum_breach(ShellLayer::Bedrock, 0.84));
        assert!(has_vacuum_breach(ShellLayer::Bedrock, 0.90));
    }
}
