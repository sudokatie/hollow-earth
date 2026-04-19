//! Hollow Earth network protocol extensions.
//!
//! Extends the base network protocol with hollow sphere-specific messages
//! for gravity synchronization, radiation, and unique entity types.

use serde::{Deserialize, Serialize};

/// Client messages specific to hollow earth mechanics.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HollowEarthClientMessage {
    /// Synchronize local gravity direction with server.
    SyncGravityDirection {
        /// Gravity direction vector (normalized).
        direction: [f32; 3],
    },

    /// Request to create a tether to another player or anchor.
    RequestTether {
        /// Target entity or anchor ID.
        target_id: u64,
    },

    /// Report current orientation state.
    SyncOrientation {
        /// Up vector relative to player.
        up_vector: [f32; 3],
        /// Current disorientation level (0.0-1.0).
        disorientation: f32,
    },

    /// Request radiation level at current position.
    QueryRadiation,

    /// Activate core containment equipment.
    ActivateContainment {
        /// Equipment slot index.
        slot: u8,
    },
}

/// Server messages specific to hollow earth mechanics.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HollowEarthServerMessage {
    /// Update a player's gravity direction.
    PlayerGravityUpdate {
        /// Player network ID.
        player_id: u64,
        /// New gravity direction (normalized).
        direction: [f32; 3],
    },

    /// Update a player's radiation exposure level.
    RadiationUpdate {
        /// Player network ID.
        player_id: u64,
        /// Current radiation level (0.0-1.0).
        level: f32,
    },

    /// Core pulse event affecting all nearby players.
    CorePulseUpdate {
        /// Core brightness intensity (0.0-1.0).
        brightness: f32,
        /// Whether a radiation storm is active.
        is_storm: bool,
    },

    /// Synchronize tether state between two anchors.
    TetherSync {
        /// First anchor position.
        anchor_a: [f32; 3],
        /// Second anchor position.
        anchor_b: [f32; 3],
    },

    /// Core exposure stage update for a player.
    ExposureStageUpdate {
        /// Player network ID.
        player_id: u64,
        /// Current exposure stage (0=Safe, 1=Caution, 2=Danger, 3=Critical, 4=Lethal).
        stage: u8,
        /// Total accumulated exposure (0.0-1.0+).
        total_exposure: f32,
    },

    /// Hollow earth entity spawned.
    HollowEntitySpawn {
        /// Entity network ID.
        entity_id: u64,
        /// Entity kind.
        kind: HollowEarthEntityKind,
        /// Position in world space.
        position: [f32; 3],
    },
}

/// Entity types unique to the hollow earth world.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HollowEarthEntityKind {
    // Hostile creatures
    /// Armored crawler that inhabits shell cavities.
    ShellCrawler,
    /// Sentient fungal colony that spreads spores.
    FungalBloom,
    /// Spectral entity drawn to core energy.
    CoreWraith,
    /// Massive serpent made of living crystal.
    CrystalSerpent,
    /// Titanic creature dwelling in the deepest depths.
    AbyssalLeviathan,

    // Passive creatures
    /// Bioluminescent beetle providing light.
    GlowBeetle,
    /// Docile grazer feeding on cavern moss.
    MossGrazer,
    /// Fish adapted to underground water bodies.
    CaveFish,
    /// Small mite living in shell material.
    ShellMite,
    /// Moth attracted to luminescent fungi.
    SporeMoth,
}

impl HollowEarthEntityKind {
    /// Check if this entity is hostile.
    #[must_use]
    pub fn is_hostile(self) -> bool {
        matches!(
            self,
            HollowEarthEntityKind::ShellCrawler
                | HollowEarthEntityKind::FungalBloom
                | HollowEarthEntityKind::CoreWraith
                | HollowEarthEntityKind::CrystalSerpent
                | HollowEarthEntityKind::AbyssalLeviathan
        )
    }

    /// Check if this entity is passive.
    #[must_use]
    pub fn is_passive(self) -> bool {
        !self.is_hostile()
    }

    /// Get the maximum health for this entity type.
    #[must_use]
    pub fn max_health(self) -> f32 {
        match self {
            // Hostile
            HollowEarthEntityKind::ShellCrawler => 25.0,
            HollowEarthEntityKind::FungalBloom => 15.0,
            HollowEarthEntityKind::CoreWraith => 40.0,
            HollowEarthEntityKind::CrystalSerpent => 100.0,
            HollowEarthEntityKind::AbyssalLeviathan => 500.0,
            // Passive
            HollowEarthEntityKind::GlowBeetle => 4.0,
            HollowEarthEntityKind::MossGrazer => 12.0,
            HollowEarthEntityKind::CaveFish => 3.0,
            HollowEarthEntityKind::ShellMite => 2.0,
            HollowEarthEntityKind::SporeMoth => 2.0,
        }
    }
}

/// Get the display name for a hollow earth entity kind.
#[must_use]
pub fn entity_kind_name(kind: &HollowEarthEntityKind) -> &'static str {
    match kind {
        HollowEarthEntityKind::ShellCrawler => "Shell Crawler",
        HollowEarthEntityKind::FungalBloom => "Fungal Bloom",
        HollowEarthEntityKind::CoreWraith => "Core Wraith",
        HollowEarthEntityKind::CrystalSerpent => "Crystal Serpent",
        HollowEarthEntityKind::AbyssalLeviathan => "Abyssal Leviathan",
        HollowEarthEntityKind::GlowBeetle => "Glow Beetle",
        HollowEarthEntityKind::MossGrazer => "Moss Grazer",
        HollowEarthEntityKind::CaveFish => "Cave Fish",
        HollowEarthEntityKind::ShellMite => "Shell Mite",
        HollowEarthEntityKind::SporeMoth => "Spore Moth",
    }
}

/// Convert gravity direction array to a normalized unit vector.
///
/// Returns a normalized direction, or zero vector if input is zero-length.
#[must_use]
pub fn normalize_gravity_direction(direction: [f32; 3]) -> [f32; 3] {
    let len_sq = direction[0] * direction[0]
        + direction[1] * direction[1]
        + direction[2] * direction[2];

    if len_sq < f32::EPSILON {
        return [0.0, 0.0, 0.0];
    }

    let len = len_sq.sqrt();
    [
        direction[0] / len,
        direction[1] / len,
        direction[2] / len,
    ]
}

/// Calculate distance between two position arrays.
#[must_use]
pub fn position_distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let dz = b[2] - a[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_kind_name() {
        assert_eq!(entity_kind_name(&HollowEarthEntityKind::ShellCrawler), "Shell Crawler");
        assert_eq!(entity_kind_name(&HollowEarthEntityKind::CoreWraith), "Core Wraith");
        assert_eq!(entity_kind_name(&HollowEarthEntityKind::GlowBeetle), "Glow Beetle");
        assert_eq!(entity_kind_name(&HollowEarthEntityKind::AbyssalLeviathan), "Abyssal Leviathan");
        assert_eq!(entity_kind_name(&HollowEarthEntityKind::SporeMoth), "Spore Moth");
    }

    #[test]
    fn test_entity_kind_hostile() {
        assert!(HollowEarthEntityKind::ShellCrawler.is_hostile());
        assert!(HollowEarthEntityKind::FungalBloom.is_hostile());
        assert!(HollowEarthEntityKind::CoreWraith.is_hostile());
        assert!(HollowEarthEntityKind::CrystalSerpent.is_hostile());
        assert!(HollowEarthEntityKind::AbyssalLeviathan.is_hostile());

        assert!(!HollowEarthEntityKind::GlowBeetle.is_hostile());
        assert!(!HollowEarthEntityKind::MossGrazer.is_hostile());
        assert!(!HollowEarthEntityKind::CaveFish.is_hostile());
    }

    #[test]
    fn test_entity_kind_passive() {
        assert!(HollowEarthEntityKind::GlowBeetle.is_passive());
        assert!(HollowEarthEntityKind::MossGrazer.is_passive());
        assert!(HollowEarthEntityKind::CaveFish.is_passive());
        assert!(HollowEarthEntityKind::ShellMite.is_passive());
        assert!(HollowEarthEntityKind::SporeMoth.is_passive());

        assert!(!HollowEarthEntityKind::ShellCrawler.is_passive());
        assert!(!HollowEarthEntityKind::CoreWraith.is_passive());
    }

    #[test]
    fn test_entity_max_health() {
        // Hostile creatures have more health
        assert!(HollowEarthEntityKind::AbyssalLeviathan.max_health() > 100.0);
        assert!(HollowEarthEntityKind::CrystalSerpent.max_health() > 50.0);

        // Passive creatures have less health
        assert!(HollowEarthEntityKind::ShellMite.max_health() < 5.0);
        assert!(HollowEarthEntityKind::GlowBeetle.max_health() < 10.0);
    }

    #[test]
    fn test_normalize_gravity_direction() {
        // Unit vector stays normalized
        let up = normalize_gravity_direction([0.0, 1.0, 0.0]);
        assert!((up[0] - 0.0).abs() < 0.001);
        assert!((up[1] - 1.0).abs() < 0.001);
        assert!((up[2] - 0.0).abs() < 0.001);

        // Non-unit vector gets normalized
        let diagonal = normalize_gravity_direction([1.0, 1.0, 1.0]);
        let expected = 1.0 / 3.0_f32.sqrt();
        assert!((diagonal[0] - expected).abs() < 0.001);
        assert!((diagonal[1] - expected).abs() < 0.001);
        assert!((diagonal[2] - expected).abs() < 0.001);

        // Zero vector returns zero
        let zero = normalize_gravity_direction([0.0, 0.0, 0.0]);
        assert_eq!(zero, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_position_distance() {
        let origin = [0.0, 0.0, 0.0];
        let point = [3.0, 4.0, 0.0];
        let distance = position_distance(origin, point);
        assert!((distance - 5.0).abs() < 0.001);

        // Same point = zero distance
        let same = position_distance(origin, origin);
        assert!((same - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_client_message_construction() {
        let msg = HollowEarthClientMessage::SyncGravityDirection {
            direction: [0.0, -1.0, 0.0],
        };

        if let HollowEarthClientMessage::SyncGravityDirection { direction } = msg {
            assert!((direction[1] - (-1.0)).abs() < 0.001);
        } else {
            panic!("Wrong message type");
        }

        let tether_msg = HollowEarthClientMessage::RequestTether { target_id: 42 };
        if let HollowEarthClientMessage::RequestTether { target_id } = tether_msg {
            assert_eq!(target_id, 42);
        }
    }

    #[test]
    fn test_server_message_construction() {
        let msg = HollowEarthServerMessage::CorePulseUpdate {
            brightness: 0.8,
            is_storm: true,
        };

        if let HollowEarthServerMessage::CorePulseUpdate { brightness, is_storm } = msg {
            assert!((brightness - 0.8).abs() < 0.001);
            assert!(is_storm);
        }

        let gravity_msg = HollowEarthServerMessage::PlayerGravityUpdate {
            player_id: 123,
            direction: [0.0, 1.0, 0.0],
        };
        if let HollowEarthServerMessage::PlayerGravityUpdate { player_id, direction } = gravity_msg {
            assert_eq!(player_id, 123);
            assert!((direction[1] - 1.0).abs() < 0.001);
        }
    }

    #[test]
    fn test_tether_sync_message() {
        let msg = HollowEarthServerMessage::TetherSync {
            anchor_a: [100.0, 0.0, 100.0],
            anchor_b: [110.0, -5.0, 100.0],
        };

        if let HollowEarthServerMessage::TetherSync { anchor_a, anchor_b } = msg {
            let distance = position_distance(anchor_a, anchor_b);
            assert!(distance < 32.0, "Tether should be within max length");
        }
    }

    #[test]
    fn test_radiation_update_message() {
        let msg = HollowEarthServerMessage::RadiationUpdate {
            player_id: 1,
            level: 0.75,
        };

        if let HollowEarthServerMessage::RadiationUpdate { player_id, level } = msg {
            assert_eq!(player_id, 1);
            assert!((level - 0.75).abs() < 0.001);
        }
    }
}
