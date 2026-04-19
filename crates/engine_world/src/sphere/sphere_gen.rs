//! Sphere surface generation for the hollow earth world.
//!
//! Converts between chunk coordinates and spherical coordinates,
//! and generates surface height variations using noise.

use noise::{NoiseFn, Perlin};
use std::f32::consts::{PI, TAU};

/// Thickness of the shell in blocks.
pub const SHELL_THICKNESS: i32 = 64;

/// Size of a chunk in blocks.
pub const CHUNK_SIZE: i32 = 16;

/// Chunk coordinates on the sphere surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SphereChunkCoords {
    /// Latitude index (north-south).
    pub lat: i32,
    /// Longitude index (east-west).
    pub lon: i32,
    /// Depth index into the shell (0 = surface, positive = deeper).
    pub depth: i32,
}

impl SphereChunkCoords {
    /// Create new sphere chunk coordinates.
    #[must_use]
    pub fn new(lat: i32, lon: i32, depth: i32) -> Self {
        Self { lat, lon, depth }
    }
}

/// Generator for sphere surface terrain.
pub struct SphereGenerator {
    /// World seed.
    pub seed: u64,
    /// Sphere radius in blocks.
    pub radius: f32,
    /// Perlin noise for surface variation.
    perlin: Perlin,
    /// Seed-based offset for noise variation.
    offset: f64,
}

impl SphereGenerator {
    /// Create a new sphere generator.
    #[must_use]
    pub fn new(seed: u64, radius: f32) -> Self {
        let perlin = Perlin::new(seed as u32);
        let offset = ((seed as f64) * 7919.0) % 100_000.0;
        Self {
            seed,
            radius,
            perlin,
            offset,
        }
    }
}

/// Generate surface height variation at a spherical position.
///
/// Returns a height offset in blocks, ranging from -8 to +8.
#[must_use]
pub fn generate_surface_height(lat: f32, lon: f32, seed: u64) -> f32 {
    let perlin = Perlin::new(seed as u32);
    let offset = ((seed as f64) * 7919.0) % 100_000.0;

    // Scale coordinates for noise sampling
    let scale = 0.1;
    let x = (lat as f64) * scale + offset;
    let y = (lon as f64) * scale + offset;

    // Sample noise and scale to +-8 blocks
    let noise_val = perlin.get([x, y]);
    (noise_val * 8.0) as f32
}

/// Convert chunk coordinates to spherical coordinates (lat/lon in radians).
///
/// Returns (latitude, longitude) where:
/// - Latitude: -PI/2 to PI/2 (south pole to north pole)
/// - Longitude: 0 to TAU (full circle)
#[must_use]
pub fn chunk_to_spherical(coords: SphereChunkCoords) -> (f32, f32) {
    // Calculate the number of chunks that span the sphere
    // Latitude spans PI radians (from -PI/2 to PI/2)
    // Longitude spans TAU radians (full circle)
    let lat_chunks = (PI * 100.0 / CHUNK_SIZE as f32).ceil() as i32;
    let lon_chunks = (TAU * 100.0 / CHUNK_SIZE as f32).ceil() as i32;

    // Convert chunk index to radians
    let lat = (coords.lat as f32 / lat_chunks as f32) * PI - PI / 2.0;
    let lon = (coords.lon as f32 / lon_chunks as f32) * TAU;

    (lat, lon)
}

/// Convert spherical coordinates (lat/lon in radians) to chunk coordinates.
///
/// Latitude should be in range -PI/2 to PI/2.
/// Longitude should be in range 0 to TAU.
#[must_use]
pub fn spherical_to_chunk(lat: f32, lon: f32) -> SphereChunkCoords {
    let lat_chunks = (PI * 100.0 / CHUNK_SIZE as f32).ceil() as i32;
    let lon_chunks = (TAU * 100.0 / CHUNK_SIZE as f32).ceil() as i32;

    // Normalize latitude to 0-1 range then to chunk index
    let lat_normalized = (lat + PI / 2.0) / PI;
    let lat_index = (lat_normalized * lat_chunks as f32).floor() as i32;

    // Normalize longitude to 0-1 range then to chunk index
    let lon_normalized = lon / TAU;
    let lon_index = (lon_normalized * lon_chunks as f32).floor() as i32;

    SphereChunkCoords::new(lat_index, lon_index, 0)
}

/// Get the four adjacent chunks on the sphere surface.
///
/// Returns neighbors in order: north, south, east, west.
#[must_use]
pub fn adjacent_chunks(coords: SphereChunkCoords) -> Vec<SphereChunkCoords> {
    let lon_chunks = (TAU * 100.0 / CHUNK_SIZE as f32).ceil() as i32;

    let north = SphereChunkCoords::new(coords.lat + 1, coords.lon, coords.depth);
    let south = SphereChunkCoords::new(coords.lat - 1, coords.lon, coords.depth);

    // Longitude wraps around
    let east_lon = (coords.lon + 1) % lon_chunks;
    let west_lon = (coords.lon - 1 + lon_chunks) % lon_chunks;

    let east = SphereChunkCoords::new(coords.lat, east_lon, coords.depth);
    let west = SphereChunkCoords::new(coords.lat, west_lon, coords.depth);

    vec![north, south, east, west]
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_sphere_chunk_coords_new() {
        let coords = SphereChunkCoords::new(10, 20, 5);
        assert_eq!(coords.lat, 10);
        assert_eq!(coords.lon, 20);
        assert_eq!(coords.depth, 5);
    }

    #[test]
    fn test_sphere_generator_new() {
        let generator = SphereGenerator::new(12345, 5000.0);
        assert_eq!(generator.seed, 12345);
        assert_relative_eq!(generator.radius, 5000.0);
    }

    #[test]
    fn test_surface_height_range() {
        for lat in 0..10 {
            for lon in 0..10 {
                let height = generate_surface_height(lat as f32 * 0.1, lon as f32 * 0.1, 42);
                assert!(height >= -8.0 && height <= 8.0,
                    "Height {} out of range at ({}, {})", height, lat, lon);
            }
        }
    }

    #[test]
    fn test_surface_height_deterministic() {
        let h1 = generate_surface_height(0.5, 0.5, 12345);
        let h2 = generate_surface_height(0.5, 0.5, 12345);
        assert_relative_eq!(h1, h2);
    }

    #[test]
    fn test_surface_height_different_seeds() {
        let h1 = generate_surface_height(0.5, 0.5, 12345);
        let h2 = generate_surface_height(0.5, 0.5, 99999);
        // Different seeds should generally produce different heights
        // (though technically could be equal by chance, it's very unlikely)
        assert!((h1 - h2).abs() > 0.001 || h1 != h2);
    }

    #[test]
    fn test_chunk_round_trip() {
        let original = SphereChunkCoords::new(5, 10, 0);
        let (lat, lon) = chunk_to_spherical(original);
        let result = spherical_to_chunk(lat, lon);

        assert_eq!(result.lat, original.lat);
        assert_eq!(result.lon, original.lon);
    }

    #[test]
    fn test_chunk_round_trip_various_positions() {
        for lat_idx in [0, 5, 10, 15] {
            for lon_idx in [0, 10, 20, 30] {
                let original = SphereChunkCoords::new(lat_idx, lon_idx, 0);
                let (lat, lon) = chunk_to_spherical(original);
                let result = spherical_to_chunk(lat, lon);

                assert_eq!(result.lat, original.lat,
                    "Lat mismatch for ({}, {})", lat_idx, lon_idx);
                assert_eq!(result.lon, original.lon,
                    "Lon mismatch for ({}, {})", lat_idx, lon_idx);
            }
        }
    }

    #[test]
    fn test_adjacent_chunks_count() {
        let coords = SphereChunkCoords::new(5, 10, 2);
        let adjacent = adjacent_chunks(coords);
        assert_eq!(adjacent.len(), 4);
    }

    #[test]
    fn test_adjacent_chunks_directions() {
        let coords = SphereChunkCoords::new(5, 10, 2);
        let adjacent = adjacent_chunks(coords);

        // North
        assert_eq!(adjacent[0].lat, 6);
        assert_eq!(adjacent[0].lon, 10);

        // South
        assert_eq!(adjacent[1].lat, 4);
        assert_eq!(adjacent[1].lon, 10);

        // East
        assert_eq!(adjacent[2].lat, 5);
        assert_eq!(adjacent[2].lon, 11);

        // West
        assert_eq!(adjacent[3].lat, 5);
        assert_eq!(adjacent[3].lon, 9);
    }

    #[test]
    fn test_adjacent_chunks_longitude_wrap() {
        let lon_chunks = (TAU * 100.0 / CHUNK_SIZE as f32).ceil() as i32;

        // Test wrap at lon = 0
        let coords = SphereChunkCoords::new(5, 0, 0);
        let adjacent = adjacent_chunks(coords);

        // West should wrap to max lon
        assert_eq!(adjacent[3].lon, lon_chunks - 1);

        // Test wrap at max lon
        let coords_max = SphereChunkCoords::new(5, lon_chunks - 1, 0);
        let adjacent_max = adjacent_chunks(coords_max);

        // East should wrap to 0
        assert_eq!(adjacent_max[2].lon, 0);
    }
}
