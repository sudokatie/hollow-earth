//! Sphere system for the hollow earth world.
//!
//! Provides sphere surface generation, biome distribution,
//! shell layers, and core region management.

pub mod biomes;
pub mod core_region;
pub mod shell;
pub mod sphere_gen;

pub use biomes::{determine_biome, biome_properties, BiomeProperties, SphereBiome};
pub use core_region::{
    core_resources, core_zone, CoreResource, CoreZone, CORE_DANGER_RADIUS, CORE_LETHAL_RADIUS,
    CORE_SAFE_RADIUS,
};
pub use shell::{has_vacuum_breach, mineral_at_depth, shell_layer, MineralType, ShellLayer, ShellProperties};
pub use sphere_gen::{
    adjacent_chunks, chunk_to_spherical, generate_surface_height, spherical_to_chunk,
    SphereChunkCoords, SphereGenerator, CHUNK_SIZE, SHELL_THICKNESS,
};
