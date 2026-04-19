//! Tether system for connecting anchor points in the hollow sphere.
//!
//! Tethers allow players to create rope connections between two anchor points,
//! useful for traversal, securing equipment, and building structures.

use glam::Vec3;
use thiserror::Error;

/// Maximum tether length in blocks.
pub const MAX_TETHER_LENGTH: f32 = 32.0;

/// Default tensile strength for a standard tether.
pub const DEFAULT_TENSILE_STRENGTH: f32 = 500.0;

/// Error type for tether operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum TetherError {
    /// Distance between anchors exceeds maximum tether length.
    #[error("tether distance exceeds maximum length of {MAX_TETHER_LENGTH} blocks")]
    TooLong,
    /// One or both anchors are in a vacuum breach zone.
    #[error("cannot attach tether to vacuum breach zone")]
    VacuumBreach,
    /// Connection between these anchors already exists.
    #[error("tether connection already exists between anchors")]
    AlreadyConnected,
}

/// A tether connecting two anchor points.
#[derive(Debug, Clone, PartialEq)]
pub struct Tether {
    /// First anchor position.
    pub anchor_a: Vec3,
    /// Second anchor position.
    pub anchor_b: Vec3,
    /// Length of the tether in blocks.
    pub length: f32,
    /// Maximum tension the tether can withstand before breaking.
    pub tensile_strength: f32,
    /// Current tension on the tether.
    pub current_tension: f32,
    /// Whether the tether has snapped.
    pub broken: bool,
}

impl Tether {
    /// Create a new tether between two positions.
    #[must_use]
    fn new(anchor_a: Vec3, anchor_b: Vec3) -> Self {
        let length = anchor_a.distance(anchor_b);
        Self {
            anchor_a,
            anchor_b,
            length,
            tensile_strength: DEFAULT_TENSILE_STRENGTH,
            current_tension: 0.0,
            broken: false,
        }
    }

    /// Create a tether with custom tensile strength.
    #[must_use]
    pub fn with_strength(mut self, strength: f32) -> Self {
        self.tensile_strength = strength;
        self
    }
}

/// An anchor point where tethers can attach.
#[derive(Debug, Clone)]
pub struct TetherAnchor {
    /// Position of the anchor in world space.
    pub position: Vec3,
    /// Indices of tethers attached to this anchor.
    pub attached_tethers: Vec<usize>,
}

impl TetherAnchor {
    /// Create a new anchor at the given position.
    #[must_use]
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            attached_tethers: Vec::new(),
        }
    }

    /// Attach a tether to this anchor.
    pub fn attach(&mut self, tether_index: usize) {
        if !self.attached_tethers.contains(&tether_index) {
            self.attached_tethers.push(tether_index);
        }
    }

    /// Detach a tether from this anchor.
    pub fn detach(&mut self, tether_index: usize) {
        self.attached_tethers.retain(|&idx| idx != tether_index);
    }
}

/// Check if a position is in a vacuum breach zone.
///
/// Vacuum breaches occur at shell boundaries where the atmosphere has escaped.
#[must_use]
pub fn is_vacuum_breach(position: Vec3) -> bool {
    // Vacuum breach zones are typically at extreme Y values (shell boundaries)
    // or in damaged sections of the sphere
    position.y < -4096.0 || position.y > 4096.0
}

/// Create a tether between two anchor positions.
///
/// # Errors
///
/// Returns an error if:
/// - Distance exceeds maximum tether length (32 blocks)
/// - Either anchor is in a vacuum breach zone
pub fn create_tether(anchor_a: Vec3, anchor_b: Vec3) -> Result<Tether, TetherError> {
    // Check vacuum breach
    if is_vacuum_breach(anchor_a) || is_vacuum_breach(anchor_b) {
        return Err(TetherError::VacuumBreach);
    }

    // Check distance
    let distance = anchor_a.distance(anchor_b);
    if distance > MAX_TETHER_LENGTH {
        return Err(TetherError::TooLong);
    }

    Ok(Tether::new(anchor_a, anchor_b))
}

/// Check if a tether is still intact.
#[must_use]
pub fn is_intact(tether: &Tether) -> bool {
    !tether.broken && tether.current_tension <= tether.tensile_strength
}

/// Apply force to a tether, increasing its tension.
///
/// If the tension exceeds tensile strength, the tether will snap.
pub fn apply_force(tether: &mut Tether, force: f32) {
    tether.current_tension += force;
    if tether.current_tension > tether.tensile_strength {
        snap_tether(tether);
    }
}

/// Snap a tether, marking it as broken.
pub fn snap_tether(tether: &mut Tether) {
    tether.broken = true;
}

/// Calculate the total length of a chain of connected tethers.
///
/// Follows connected tethers through anchors to sum their lengths.
#[must_use]
pub fn chain_length(anchors: &[TetherAnchor], tethers: &[Tether]) -> f32 {
    let mut total = 0.0;
    let mut visited = vec![false; tethers.len()];

    for anchor in anchors {
        for &tether_idx in &anchor.attached_tethers {
            if tether_idx < tethers.len() && !visited[tether_idx] {
                let tether = &tethers[tether_idx];
                if !tether.broken {
                    total += tether.length;
                    visited[tether_idx] = true;
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tether_valid() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(10.0, 0.0, 0.0);
        let result = create_tether(a, b);
        assert!(result.is_ok(), "should create tether within max length");
        let tether = result.unwrap();
        assert!((tether.length - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_create_tether_too_long() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(50.0, 0.0, 0.0);
        let result = create_tether(a, b);
        assert_eq!(result, Err(TetherError::TooLong));
    }

    #[test]
    fn test_create_tether_vacuum_breach() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 5000.0, 0.0);
        let result = create_tether(a, b);
        assert_eq!(result, Err(TetherError::VacuumBreach));
    }

    #[test]
    fn test_is_intact_healthy_tether() {
        let tether = Tether::new(Vec3::ZERO, Vec3::new(5.0, 0.0, 0.0));
        assert!(is_intact(&tether), "new tether should be intact");
    }

    #[test]
    fn test_is_intact_broken_tether() {
        let mut tether = Tether::new(Vec3::ZERO, Vec3::new(5.0, 0.0, 0.0));
        snap_tether(&mut tether);
        assert!(!is_intact(&tether), "snapped tether should not be intact");
    }

    #[test]
    fn test_apply_force_within_strength() {
        let mut tether = Tether::new(Vec3::ZERO, Vec3::new(5.0, 0.0, 0.0));
        apply_force(&mut tether, 100.0);
        assert!(is_intact(&tether), "should remain intact under strength");
        assert!((tether.current_tension - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_apply_force_exceeds_strength() {
        let mut tether = Tether::new(Vec3::ZERO, Vec3::new(5.0, 0.0, 0.0));
        apply_force(&mut tether, 600.0);
        assert!(!is_intact(&tether), "should break when force exceeds strength");
        assert!(tether.broken);
    }

    #[test]
    fn test_chain_length_single() {
        let tether = Tether::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0));
        let mut anchor = TetherAnchor::new(Vec3::ZERO);
        anchor.attach(0);

        let total = chain_length(&[anchor], &[tether]);
        assert!((total - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_chain_length_multiple() {
        let tether1 = Tether::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0));
        let tether2 = Tether::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(20.0, 0.0, 0.0));

        let mut anchor1 = TetherAnchor::new(Vec3::ZERO);
        anchor1.attach(0);
        let mut anchor2 = TetherAnchor::new(Vec3::new(10.0, 0.0, 0.0));
        anchor2.attach(0);
        anchor2.attach(1);
        let mut anchor3 = TetherAnchor::new(Vec3::new(20.0, 0.0, 0.0));
        anchor3.attach(1);

        let total = chain_length(&[anchor1, anchor2, anchor3], &[tether1, tether2]);
        assert!((total - 20.0).abs() < 0.001);
    }

    #[test]
    fn test_anchor_attach_detach() {
        let mut anchor = TetherAnchor::new(Vec3::ZERO);
        anchor.attach(0);
        anchor.attach(1);
        assert_eq!(anchor.attached_tethers.len(), 2);

        anchor.detach(0);
        assert_eq!(anchor.attached_tethers.len(), 1);
        assert_eq!(anchor.attached_tethers[0], 1);
    }
}
