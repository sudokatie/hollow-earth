//! Core glow and pulse system for the hollow earth world.
//!
//! The central core provides light to the interior surface, following a
//! day/night cycle through intensity changes and occasional storm events.

use std::f32::consts::{PI, TAU};

/// Radius of the core in blocks.
pub const CORE_RADIUS: f32 = 512.0;

/// Core state tracking day/night cycle and storm events.
#[derive(Debug, Clone, Copy)]
pub struct CoreState {
    /// Core intensity multiplier (0.0-1.0 normal, up to 2.0 in storm).
    pub intensity: f32,
    /// Current phase of the day/night cycle (0.0-1.0).
    /// 0.0-0.5 = day, 0.5-1.0 = night.
    pub pulse_phase: f32,
    /// Whether a storm event is active.
    pub is_storm: bool,
    /// Duration of a full day/night cycle in seconds.
    pub day_duration: f32,
}

impl Default for CoreState {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            pulse_phase: 0.0,
            is_storm: false,
            day_duration: 600.0,
        }
    }
}

impl CoreState {
    /// Create a new core state with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a core state at a specific phase.
    #[must_use]
    pub fn at_phase(phase: f32) -> Self {
        Self {
            pulse_phase: phase.rem_euclid(1.0),
            ..Default::default()
        }
    }
}

/// Update the core state, advancing the pulse phase.
///
/// Returns a new `CoreState` with updated phase and intensity.
#[must_use]
pub fn update_core(state: CoreState, delta: f32) -> CoreState {
    let phase_delta = delta / state.day_duration;
    let new_phase = (state.pulse_phase + phase_delta).rem_euclid(1.0);

    CoreState {
        pulse_phase: new_phase,
        intensity: core_brightness(CoreState {
            pulse_phase: new_phase,
            ..state
        }),
        ..state
    }
}

/// Calculate the current core brightness based on state.
///
/// Returns brightness in range 0.2-1.0 (or up to 2.0 during storms).
/// - Day (phase 0.0-0.5): 100% brightness
/// - Night (phase 0.5-1.0): 20% brightness
/// - Smooth sine transition between phases
#[must_use]
pub fn core_brightness(state: CoreState) -> f32 {
    if state.is_storm {
        return 2.0;
    }

    // Use sine wave for smooth transition
    // Phase 0.0 = sunrise, 0.25 = noon, 0.5 = sunset, 0.75 = midnight
    // sin(0) = 0, sin(PI/2) = 1, sin(PI) = 0, sin(3PI/2) = -1
    let angle = state.pulse_phase * TAU;
    let sine_value = angle.sin();

    // Map sine (-1 to 1) to brightness (0.2 to 1.0)
    // At phase 0.25: sin(PI/2) = 1 -> brightness = 1.0
    // At phase 0.75: sin(3PI/2) = -1 -> brightness = 0.2
    let brightness = 0.6 + 0.4 * sine_value;

    brightness.clamp(0.2, 1.0)
}

/// Calculate core color based on brightness level.
///
/// Returns warm white/yellow color that shifts redder at low brightness.
#[must_use]
pub fn core_color(brightness: f32) -> [f32; 3] {
    // At full brightness: warm white/yellow (1.0, 0.95, 0.8)
    // At low brightness: redder (1.0, 0.6, 0.3)

    let t = brightness.clamp(0.2, 1.0);

    // Normalize t to 0-1 range based on brightness range
    let t_normalized = (t - 0.2) / 0.8;

    // Interpolate between red-ish (low) and warm white (high)
    let r = 1.0;
    let g = 0.6 + 0.35 * t_normalized;
    let b = 0.3 + 0.5 * t_normalized;

    [r, g, b]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_core_state() {
        let state = CoreState::default();
        assert!((state.intensity - 1.0).abs() < 0.001);
        assert!((state.pulse_phase).abs() < 0.001);
        assert!(!state.is_storm);
        assert!((state.day_duration - 600.0).abs() < 0.001);
    }

    #[test]
    fn test_core_radius_constant() {
        assert!((CORE_RADIUS - 512.0).abs() < 0.001);
    }

    #[test]
    fn test_update_core_advances_phase() {
        let state = CoreState::default();
        let updated = update_core(state, 60.0); // 60 seconds = 10% of default day

        assert!((updated.pulse_phase - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_update_core_wraps_phase() {
        let state = CoreState {
            pulse_phase: 0.95,
            ..Default::default()
        };
        let updated = update_core(state, 60.0); // 10% more, should wrap

        assert!(updated.pulse_phase < 0.1, "Phase should wrap at 1.0");
        assert!(updated.pulse_phase >= 0.0);
    }

    #[test]
    fn test_brightness_at_noon() {
        let state = CoreState {
            pulse_phase: 0.25, // Noon
            is_storm: false,
            ..Default::default()
        };
        let brightness = core_brightness(state);

        assert!(brightness > 0.95, "Noon should be near full brightness");
    }

    #[test]
    fn test_brightness_at_midnight() {
        let state = CoreState {
            pulse_phase: 0.75, // Midnight
            is_storm: false,
            ..Default::default()
        };
        let brightness = core_brightness(state);

        assert!(brightness < 0.25, "Midnight should be near minimum brightness");
    }

    #[test]
    fn test_brightness_storm_override() {
        let state = CoreState {
            pulse_phase: 0.75, // Midnight but storm
            is_storm: true,
            ..Default::default()
        };
        let brightness = core_brightness(state);

        assert!((brightness - 2.0).abs() < 0.001, "Storm should be 200% brightness");
    }

    #[test]
    fn test_brightness_range() {
        for phase in 0..100 {
            let state = CoreState {
                pulse_phase: phase as f32 / 100.0,
                is_storm: false,
                ..Default::default()
            };
            let brightness = core_brightness(state);

            assert!(brightness >= 0.2, "Brightness should never go below 0.2");
            assert!(brightness <= 1.0, "Brightness should never exceed 1.0 without storm");
        }
    }

    #[test]
    fn test_core_color_warm_at_full() {
        let color = core_color(1.0);

        assert!((color[0] - 1.0).abs() < 0.01, "Red should be full");
        assert!(color[1] > 0.9, "Green should be high");
        assert!(color[2] > 0.7, "Blue should be moderate");
    }

    #[test]
    fn test_core_color_redder_at_low() {
        let color_low = core_color(0.2);
        let color_high = core_color(1.0);

        assert!(
            color_low[1] < color_high[1],
            "Green should be lower at low brightness"
        );
        assert!(
            color_low[2] < color_high[2],
            "Blue should be lower at low brightness"
        );
    }
}
