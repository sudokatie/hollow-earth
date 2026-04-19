//! Biome distribution system for the hollow earth sphere.
//!
//! Determines biome types based on spherical position and provides
//! properties for each biome type.

use noise::{NoiseFn, Perlin};
use std::f32::consts::PI;

/// Biome types on the sphere interior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SphereBiome {
    /// Lush moss-covered plains near spawn.
    MossPlains,
    /// Dense bioluminescent fungal forests.
    FungalForest,
    /// Caverns filled with glowing crystals.
    CrystalCaverns,
    /// Hot volcanic regions with lava flows.
    MagmaFields,
    /// Deep dark chasms with minimal light.
    DeepChasm,
    /// Dangerous zone near the core with radiation.
    CoreProximity,
}

/// Properties of a biome.
#[derive(Clone, Copy, Debug)]
pub struct BiomeProperties {
    /// Ambient temperature (0.0 = freezing, 1.0 = scorching).
    pub ambient_temp: f32,
    /// Natural light level (0.0 = pitch black, 1.0 = bright).
    pub light_level: f32,
    /// Danger level (0.0 = safe, 1.0 = deadly).
    pub danger_level: f32,
    /// Resource richness (0.0 = barren, 1.0 = abundant).
    pub resources: f32,
}

impl BiomeProperties {
    /// Create new biome properties.
    #[must_use]
    pub const fn new(ambient_temp: f32, light_level: f32, danger_level: f32, resources: f32) -> Self {
        Self {
            ambient_temp,
            light_level,
            danger_level,
            resources,
        }
    }
}

/// Determine the biome at a spherical position.
///
/// Uses noise-based selection with special rules:
/// - MossPlains favored near center of map (spawn area)
/// - CoreProximity appears at specific depth/distance thresholds
#[must_use]
pub fn determine_biome(lat: f32, lon: f32, seed: u64) -> SphereBiome {
    // Create noise generators for biome selection
    let biome_noise = Perlin::new(seed as u32);
    let secondary_noise = Perlin::new(seed.wrapping_mul(31337) as u32);
    let offset = ((seed as f64) * 5749.0) % 100_000.0;

    // Calculate distance from center of map (spawn point at lat=0, lon=PI)
    let spawn_lat = 0.0_f32;
    let spawn_lon = PI;
    let lat_diff = (lat - spawn_lat).abs();
    let lon_diff = (lon - spawn_lon).abs().min(PI * 2.0 - (lon - spawn_lon).abs());
    let dist_from_spawn = (lat_diff * lat_diff + lon_diff * lon_diff).sqrt();

    // Near spawn = MossPlains (safe starting area)
    if dist_from_spawn < 0.3 {
        return SphereBiome::MossPlains;
    }

    // Sample noise for biome determination
    let scale = 0.5;
    let x = (lat as f64) * scale + offset;
    let y = (lon as f64) * scale + offset;

    let primary = biome_noise.get([x, y]);
    let secondary = secondary_noise.get([x * 1.5, y * 1.5 + 500.0]);

    // Far from spawn with specific noise = CoreProximity (rare, dangerous)
    if dist_from_spawn > 2.5 && primary > 0.7 && secondary > 0.5 {
        return SphereBiome::CoreProximity;
    }

    // Select biome based on noise values
    let combined = (primary + 1.0) / 2.0; // Normalize to 0-1

    if combined < 0.2 {
        SphereBiome::DeepChasm
    } else if combined < 0.35 {
        SphereBiome::MagmaFields
    } else if combined < 0.55 {
        SphereBiome::CrystalCaverns
    } else if combined < 0.75 {
        SphereBiome::FungalForest
    } else {
        SphereBiome::MossPlains
    }
}

/// Get the properties for a given biome.
#[must_use]
pub fn biome_properties(biome: SphereBiome) -> BiomeProperties {
    match biome {
        SphereBiome::MossPlains => BiomeProperties::new(
            0.5,  // Comfortable temperature
            0.6,  // Good ambient light from bioluminescence
            0.1,  // Very safe
            0.4,  // Moderate resources
        ),
        SphereBiome::FungalForest => BiomeProperties::new(
            0.55, // Slightly warm from decay
            0.7,  // Bright bioluminescent fungi
            0.2,  // Some hostile creatures
            0.6,  // Good resources
        ),
        SphereBiome::CrystalCaverns => BiomeProperties::new(
            0.4,  // Cool
            0.8,  // Very bright from crystals
            0.3,  // Moderate danger
            0.9,  // Excellent mineral resources
        ),
        SphereBiome::MagmaFields => BiomeProperties::new(
            0.95, // Extremely hot
            0.5,  // Dim red glow from lava
            0.7,  // Very dangerous
            0.7,  // Good rare resources
        ),
        SphereBiome::DeepChasm => BiomeProperties::new(
            0.3,  // Cold
            0.1,  // Nearly pitch black
            0.6,  // Dangerous predators
            0.3,  // Sparse resources
        ),
        SphereBiome::CoreProximity => BiomeProperties::new(
            0.85, // Hot from core radiation
            0.9,  // Bright core glow
            1.0,  // Maximum danger (radiation)
            1.0,  // Rarest resources
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_biome_determination_at_spawn() {
        // Near spawn should be MossPlains
        let biome = determine_biome(0.0, PI, 12345);
        assert_eq!(biome, SphereBiome::MossPlains);
    }

    #[test]
    fn test_biome_determination_deterministic() {
        let b1 = determine_biome(1.0, 2.0, 42);
        let b2 = determine_biome(1.0, 2.0, 42);
        assert_eq!(b1, b2);
    }

    #[test]
    fn test_biome_determination_different_seeds() {
        // Sample many positions - different seeds should produce different patterns
        let mut same_count = 0;
        for i in 0..20 {
            let lat = i as f32 * 0.15;
            let lon = i as f32 * 0.2;
            let b1 = determine_biome(lat, lon, 12345);
            let b2 = determine_biome(lat, lon, 99999);
            if b1 == b2 {
                same_count += 1;
            }
        }
        // Not all should be the same (statistically very unlikely)
        assert!(same_count < 20, "All biomes were identical with different seeds");
    }

    #[test]
    fn test_biome_transitions() {
        // Biomes should transition smoothly (nearby points often share biomes)
        let seed = 42_u64;
        let base_biome = determine_biome(1.0, 1.0, seed);

        let mut neighbor_matches = 0;
        for delta in [0.01_f32, 0.02, 0.03] {
            let neighbor = determine_biome(1.0 + delta, 1.0, seed);
            if neighbor == base_biome {
                neighbor_matches += 1;
            }
        }

        // Close neighbors should often have the same biome
        assert!(neighbor_matches >= 1, "No smooth transitions detected");
    }

    #[test]
    fn test_moss_plains_properties() {
        let props = biome_properties(SphereBiome::MossPlains);
        assert_relative_eq!(props.ambient_temp, 0.5);
        assert_relative_eq!(props.light_level, 0.6);
        assert_relative_eq!(props.danger_level, 0.1);
        assert_relative_eq!(props.resources, 0.4);
    }

    #[test]
    fn test_fungal_forest_properties() {
        let props = biome_properties(SphereBiome::FungalForest);
        assert_relative_eq!(props.ambient_temp, 0.55);
        assert_relative_eq!(props.light_level, 0.7);
        assert_relative_eq!(props.danger_level, 0.2);
        assert_relative_eq!(props.resources, 0.6);
    }

    #[test]
    fn test_crystal_caverns_properties() {
        let props = biome_properties(SphereBiome::CrystalCaverns);
        assert!(props.light_level > 0.7, "Crystal caverns should be bright");
        assert!(props.resources > 0.8, "Crystal caverns should be resource-rich");
    }

    #[test]
    fn test_core_proximity_properties() {
        let props = biome_properties(SphereBiome::CoreProximity);
        assert_relative_eq!(props.danger_level, 1.0);
        assert_relative_eq!(props.resources, 1.0);
        assert!(props.ambient_temp > 0.8, "Core should be hot");
    }
}
