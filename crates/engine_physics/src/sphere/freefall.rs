//! Freefall physics for hollow sphere world.
//!
//! Handles falling through the sphere interior, terminal velocity,
//! landing detection, and fall damage.

use glam::Vec3;

/// Terminal velocity in meters per second.
pub const TERMINAL_VELOCITY: f32 = 100.0;

/// Gravity acceleration in m/s^2.
const GRAVITY_ACCELERATION: f32 = 9.81;

/// Safe landing velocity threshold (no damage below this).
const SAFE_VELOCITY: f32 = 10.0;

/// Damage multiplier per m/s above safe velocity.
const DAMAGE_PER_VELOCITY: f32 = 2.0;

/// Current freefall state.
#[derive(Debug, Clone, Copy)]
pub struct FreefallState {
    /// Current position.
    pub position: Vec3,
    /// Current velocity.
    pub velocity: Vec3,
    /// Position where freefall started.
    pub source_position: Vec3,
}

impl FreefallState {
    /// Create a new freefall state.
    #[must_use]
    pub fn new(position: Vec3, velocity: Vec3) -> Self {
        Self {
            position,
            velocity,
            source_position: position,
        }
    }

    /// Create a new freefall state with custom source position.
    #[must_use]
    pub fn with_source(position: Vec3, velocity: Vec3, source_position: Vec3) -> Self {
        Self {
            position,
            velocity,
            source_position,
        }
    }
}

/// Result of a freefall physics update.
#[derive(Debug, Clone, Copy)]
pub struct FreefallResult {
    /// New position after update.
    pub new_position: Vec3,
    /// New velocity after update.
    pub new_velocity: Vec3,
    /// Whether the player has landed.
    pub has_landed: bool,
    /// Landing velocity magnitude (0.0 if not landed).
    pub landing_velocity: f32,
}

/// Fall arrestor equipment that can stop a fall.
#[derive(Debug, Clone, Copy)]
pub struct FallArrestor {
    /// Whether the arrestor is active.
    pub is_active: bool,
    /// Remaining charges.
    pub charges: u32,
}

impl FallArrestor {
    /// Create a new fall arrestor with charges.
    #[must_use]
    pub fn new(charges: u32) -> Self {
        Self {
            is_active: false,
            charges,
        }
    }

    /// Check if the arrestor has charges remaining.
    #[must_use]
    pub fn has_charges(&self) -> bool {
        self.charges > 0
    }

    /// Activate the fall arrestor, consuming one charge.
    ///
    /// Returns true if activation succeeded.
    pub fn activate(&mut self) -> bool {
        if self.charges > 0 {
            self.charges -= 1;
            self.is_active = true;
            true
        } else {
            false
        }
    }

    /// Reset the active state.
    pub fn reset(&mut self) {
        self.is_active = false;
    }
}

impl Default for FallArrestor {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Update freefall physics.
///
/// In the hollow sphere, gravity points outward from the center toward
/// the inner surface. When falling, the player accelerates toward the
/// opposite surface.
#[must_use]
pub fn update_freefall(
    state: &FreefallState,
    delta: f32,
    sphere_center: Vec3,
    sphere_radius: f32,
) -> FreefallResult {
    // Calculate gravity direction (outward from center)
    let to_center = sphere_center - state.position;
    let distance_to_center = to_center.length();

    // If at center, no gravity direction
    let gravity_dir = if distance_to_center > 0.001 {
        -to_center.normalize() // Outward from center
    } else {
        Vec3::ZERO
    };

    // Apply gravity acceleration
    let acceleration = gravity_dir * GRAVITY_ACCELERATION;
    let mut new_velocity = state.velocity + acceleration * delta;

    // Cap at terminal velocity
    let speed = new_velocity.length();
    if speed > TERMINAL_VELOCITY {
        new_velocity = new_velocity.normalize() * TERMINAL_VELOCITY;
    }

    // Update position
    let new_position = state.position + new_velocity * delta;

    // Check for landing (reached sphere surface)
    let new_distance = (new_position - sphere_center).length();
    let has_landed = new_distance >= sphere_radius;

    // Calculate landing velocity
    let landing_velocity = if has_landed { speed } else { 0.0 };

    // Clamp position to sphere surface if landed
    let final_position = if has_landed {
        let dir = (new_position - sphere_center).normalize_or_zero();
        sphere_center + dir * sphere_radius
    } else {
        new_position
    };

    // Stop velocity if landed
    let final_velocity = if has_landed { Vec3::ZERO } else { new_velocity };

    FreefallResult {
        new_position: final_position,
        new_velocity: final_velocity,
        has_landed,
        landing_velocity,
    }
}

/// Calculate fall damage based on landing velocity.
///
/// Damage = (velocity - 10).max(0) * 2 HP
#[must_use]
pub fn fall_damage(landing_velocity: f32) -> f32 {
    (landing_velocity - SAFE_VELOCITY).max(0.0) * DAMAGE_PER_VELOCITY
}

#[cfg(test)]
mod tests {
    use super::*;

    const CENTER: Vec3 = Vec3::ZERO;
    const RADIUS: f32 = 4096.0;

    #[test]
    fn test_freefall_accelerates_toward_surface() {
        let state = FreefallState::new(Vec3::new(100.0, 0.0, 0.0), Vec3::ZERO);
        let result = update_freefall(&state, 1.0, CENTER, RADIUS);

        // Should accelerate outward (positive X)
        assert!(result.new_velocity.x > 0.0);
    }

    #[test]
    fn test_freefall_terminal_velocity() {
        let fast_velocity = Vec3::new(200.0, 0.0, 0.0);
        let state = FreefallState::new(Vec3::new(100.0, 0.0, 0.0), fast_velocity);
        let result = update_freefall(&state, 1.0, CENTER, RADIUS);

        let speed = result.new_velocity.length();
        assert!(speed <= TERMINAL_VELOCITY + 0.001);
    }

    #[test]
    fn test_freefall_landing_detection() {
        let position = Vec3::new(RADIUS - 5.0, 0.0, 0.0);
        let velocity = Vec3::new(50.0, 0.0, 0.0);
        let state = FreefallState::new(position, velocity);
        let result = update_freefall(&state, 1.0, CENTER, RADIUS);

        assert!(result.has_landed);
        assert!(result.landing_velocity > 0.0);
    }

    #[test]
    fn test_freefall_no_landing_mid_fall() {
        let position = Vec3::new(2000.0, 0.0, 0.0);
        let velocity = Vec3::new(10.0, 0.0, 0.0);
        let state = FreefallState::new(position, velocity);
        let result = update_freefall(&state, 0.1, CENTER, RADIUS);

        assert!(!result.has_landed);
        assert!((result.landing_velocity - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_fall_damage_below_safe() {
        let damage = fall_damage(5.0);
        assert!((damage - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_fall_damage_at_safe_threshold() {
        let damage = fall_damage(10.0);
        assert!((damage - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_fall_damage_above_safe() {
        let damage = fall_damage(20.0);
        // (20 - 10) * 2 = 20 HP
        assert!((damage - 20.0).abs() < 0.001);
    }

    #[test]
    fn test_fall_arrestor_has_charges() {
        let arrestor = FallArrestor::new(3);
        assert!(arrestor.has_charges());
    }

    #[test]
    fn test_fall_arrestor_activate() {
        let mut arrestor = FallArrestor::new(2);
        assert!(arrestor.activate());
        assert!(arrestor.is_active);
        assert_eq!(arrestor.charges, 1);
    }

    #[test]
    fn test_fall_arrestor_no_charges() {
        let mut arrestor = FallArrestor::new(0);
        assert!(!arrestor.has_charges());
        assert!(!arrestor.activate());
    }
}
