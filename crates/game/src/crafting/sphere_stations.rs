//! Sphere-specific crafting stations for hollow earth world.
//!
//! Specialized stations that may require specific biomes or zones to function.

use engine_world::sphere::{biomes::SphereBiome, core_region::CoreZone};

/// Types of sphere-specific crafting stations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SphereStationType {
    /// Basic station for crafting from fungal materials.
    FungalWorkbench,
    /// Advanced station for crystal-based items.
    CrystalForge,
    /// Endgame station for core energy items.
    CoreKiln,
    /// Station for processing fibers.
    SpinningVat,
}

/// Materials required for crafting at sphere stations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialType {
    /// Fibers harvested from fungi.
    FungalFiber,
    /// Fragments from the shell layer.
    ShellFragment,
    /// Crystals from caverns.
    Crystal,
    /// Fragments from the core.
    CoreFragment,
    /// High-energy crystals from danger zones.
    EnergyCrystal,
    /// Condensed plasma from the core.
    PlasmaOrb,
    /// Silk from creatures.
    Silk,
}

/// Requirements for placing and using a sphere station.
#[derive(Debug, Clone)]
pub struct StationRequirements {
    /// Biome restriction (None = can be placed anywhere).
    pub biome_restriction: Option<SphereBiome>,
    /// Zone restriction (None = can be placed anywhere).
    pub zone_restriction: Option<CoreZone>,
    /// Materials required to build the station.
    pub materials: Vec<MaterialType>,
}

impl StationRequirements {
    /// Create requirements with no restrictions.
    #[must_use]
    pub fn new() -> Self {
        Self {
            biome_restriction: None,
            zone_restriction: None,
            materials: Vec::new(),
        }
    }

    /// Set biome restriction.
    #[must_use]
    pub fn with_biome(mut self, biome: SphereBiome) -> Self {
        self.biome_restriction = Some(biome);
        self
    }

    /// Set zone restriction.
    #[must_use]
    pub fn with_zone(mut self, zone: CoreZone) -> Self {
        self.zone_restriction = Some(zone);
        self
    }

    /// Add required materials.
    #[must_use]
    pub fn with_materials(mut self, materials: Vec<MaterialType>) -> Self {
        self.materials = materials;
        self
    }

    /// Check if requirements are met at a given location.
    #[must_use]
    pub fn can_place(&self, biome: SphereBiome, zone: CoreZone) -> bool {
        let biome_ok = self.biome_restriction.map_or(true, |req| req == biome);
        let zone_ok = self.zone_restriction.map_or(true, |req| req == zone);
        biome_ok && zone_ok
    }

    /// Check if player has required materials.
    #[must_use]
    pub fn has_materials(&self, available: &[MaterialType]) -> bool {
        self.materials.iter().all(|req| available.contains(req))
    }
}

impl Default for StationRequirements {
    fn default() -> Self {
        Self::new()
    }
}

/// Fungal workbench for basic crafting.
#[derive(Debug, Clone)]
pub struct FungalWorkbench {
    /// Station durability.
    pub durability: f32,
}

impl FungalWorkbench {
    /// Create a new fungal workbench.
    #[must_use]
    pub fn new() -> Self {
        Self { durability: 100.0 }
    }

    /// Get station requirements.
    #[must_use]
    pub fn requirements() -> StationRequirements {
        StationRequirements::new()
            .with_materials(vec![MaterialType::FungalFiber, MaterialType::ShellFragment])
    }

    /// Get station type.
    #[must_use]
    pub fn station_type() -> SphereStationType {
        SphereStationType::FungalWorkbench
    }
}

impl Default for FungalWorkbench {
    fn default() -> Self {
        Self::new()
    }
}

/// Crystal forge for advanced items.
///
/// Must be placed near Crystal Caverns biome.
#[derive(Debug, Clone)]
pub struct CrystalForge {
    /// Station durability.
    pub durability: f32,
    /// Current heat level (0.0 to 1.0).
    pub heat: f32,
}

impl CrystalForge {
    /// Create a new crystal forge.
    #[must_use]
    pub fn new() -> Self {
        Self {
            durability: 150.0,
            heat: 0.0,
        }
    }

    /// Get station requirements.
    #[must_use]
    pub fn requirements() -> StationRequirements {
        StationRequirements::new()
            .with_biome(SphereBiome::CrystalCaverns)
            .with_materials(vec![MaterialType::Crystal, MaterialType::CoreFragment])
    }

    /// Get station type.
    #[must_use]
    pub fn station_type() -> SphereStationType {
        SphereStationType::CrystalForge
    }

    /// Heat up the forge.
    pub fn heat_up(&mut self, amount: f32) {
        self.heat = (self.heat + amount).min(1.0);
    }

    /// Cool down the forge.
    pub fn cool_down(&mut self, amount: f32) {
        self.heat = (self.heat - amount).max(0.0);
    }
}

impl Default for CrystalForge {
    fn default() -> Self {
        Self::new()
    }
}

/// Core kiln for endgame crafting.
///
/// Must be placed in the Danger zone (distance < 2048 from center).
#[derive(Debug, Clone)]
pub struct CoreKiln {
    /// Station durability.
    pub durability: f32,
    /// Core energy level.
    pub core_energy: f32,
}

impl CoreKiln {
    /// Maximum core energy capacity.
    pub const MAX_CORE_ENERGY: f32 = 100.0;

    /// Create a new core kiln.
    #[must_use]
    pub fn new() -> Self {
        Self {
            durability: 200.0,
            core_energy: 0.0,
        }
    }

    /// Get station requirements.
    #[must_use]
    pub fn requirements() -> StationRequirements {
        StationRequirements::new()
            .with_zone(CoreZone::Danger)
            .with_materials(vec![
                MaterialType::CoreFragment,
                MaterialType::EnergyCrystal,
                MaterialType::PlasmaOrb,
            ])
    }

    /// Get station type.
    #[must_use]
    pub fn station_type() -> SphereStationType {
        SphereStationType::CoreKiln
    }

    /// Charge the kiln with core energy.
    pub fn charge(&mut self, amount: f32) {
        self.core_energy = (self.core_energy + amount).min(Self::MAX_CORE_ENERGY);
    }

    /// Consume core energy for crafting.
    ///
    /// Returns true if sufficient energy was available.
    pub fn consume_energy(&mut self, amount: f32) -> bool {
        if self.core_energy >= amount {
            self.core_energy -= amount;
            true
        } else {
            false
        }
    }
}

impl Default for CoreKiln {
    fn default() -> Self {
        Self::new()
    }
}

/// Spinning vat for processing fibers.
#[derive(Debug, Clone)]
pub struct SpinningVat {
    /// Station durability.
    pub durability: f32,
    /// Items currently being processed.
    pub processing: bool,
    /// Time remaining on current process.
    pub process_time: f32,
}

impl SpinningVat {
    /// Default processing time in seconds.
    pub const DEFAULT_PROCESS_TIME: f32 = 10.0;

    /// Create a new spinning vat.
    #[must_use]
    pub fn new() -> Self {
        Self {
            durability: 80.0,
            processing: false,
            process_time: 0.0,
        }
    }

    /// Get station requirements.
    #[must_use]
    pub fn requirements() -> StationRequirements {
        StationRequirements::new()
            .with_materials(vec![MaterialType::FungalFiber])
    }

    /// Get station type.
    #[must_use]
    pub fn station_type() -> SphereStationType {
        SphereStationType::SpinningVat
    }

    /// Start processing fibers.
    pub fn start_processing(&mut self) {
        self.processing = true;
        self.process_time = Self::DEFAULT_PROCESS_TIME;
    }

    /// Update processing state.
    ///
    /// Returns true if processing completed this frame.
    pub fn update(&mut self, delta: f32) -> bool {
        if self.processing {
            self.process_time -= delta;
            if self.process_time <= 0.0 {
                self.processing = false;
                self.process_time = 0.0;
                return true;
            }
        }
        false
    }

    /// Get processing progress (0.0 to 1.0).
    #[must_use]
    pub fn progress(&self) -> f32 {
        if self.processing {
            1.0 - (self.process_time / Self::DEFAULT_PROCESS_TIME)
        } else {
            0.0
        }
    }
}

impl Default for SpinningVat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fungal_workbench_requirements() {
        let req = FungalWorkbench::requirements();
        assert!(req.biome_restriction.is_none());
        assert!(req.zone_restriction.is_none());
        assert!(req.materials.contains(&MaterialType::FungalFiber));
        assert!(req.materials.contains(&MaterialType::ShellFragment));
    }

    #[test]
    fn test_crystal_forge_biome_restriction() {
        let req = CrystalForge::requirements();
        assert_eq!(req.biome_restriction, Some(SphereBiome::CrystalCaverns));

        // Can place in Crystal Caverns
        assert!(req.can_place(SphereBiome::CrystalCaverns, CoreZone::Safe));
        // Cannot place in other biomes
        assert!(!req.can_place(SphereBiome::MossPlains, CoreZone::Safe));
    }

    #[test]
    fn test_core_kiln_zone_restriction() {
        let req = CoreKiln::requirements();
        assert_eq!(req.zone_restriction, Some(CoreZone::Danger));

        // Can place in Danger zone
        assert!(req.can_place(SphereBiome::CoreProximity, CoreZone::Danger));
        // Cannot place in Safe zone
        assert!(!req.can_place(SphereBiome::CoreProximity, CoreZone::Safe));
    }

    #[test]
    fn test_station_requirements_materials() {
        let req = CoreKiln::requirements();
        let available = vec![
            MaterialType::CoreFragment,
            MaterialType::EnergyCrystal,
            MaterialType::PlasmaOrb,
        ];
        assert!(req.has_materials(&available));

        let missing = vec![MaterialType::CoreFragment];
        assert!(!req.has_materials(&missing));
    }

    #[test]
    fn test_core_kiln_energy() {
        let mut kiln = CoreKiln::new();
        kiln.charge(50.0);
        assert!((kiln.core_energy - 50.0).abs() < 0.001);

        assert!(kiln.consume_energy(30.0));
        assert!((kiln.core_energy - 20.0).abs() < 0.001);

        assert!(!kiln.consume_energy(30.0)); // Not enough energy
    }

    #[test]
    fn test_spinning_vat_processing() {
        let mut vat = SpinningVat::new();
        vat.start_processing();
        assert!(vat.processing);

        // Process halfway
        let completed = vat.update(5.0);
        assert!(!completed);
        assert!((vat.progress() - 0.5).abs() < 0.001);

        // Complete processing
        let completed = vat.update(6.0);
        assert!(completed);
        assert!(!vat.processing);
    }

    #[test]
    fn test_crystal_forge_heat() {
        let mut forge = CrystalForge::new();
        forge.heat_up(0.5);
        assert!((forge.heat - 0.5).abs() < 0.001);

        forge.heat_up(0.8); // Should cap at 1.0
        assert!((forge.heat - 1.0).abs() < 0.001);

        forge.cool_down(0.3);
        assert!((forge.heat - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_station_types() {
        assert_eq!(FungalWorkbench::station_type(), SphereStationType::FungalWorkbench);
        assert_eq!(CrystalForge::station_type(), SphereStationType::CrystalForge);
        assert_eq!(CoreKiln::station_type(), SphereStationType::CoreKiln);
        assert_eq!(SpinningVat::station_type(), SphereStationType::SpinningVat);
    }
}
