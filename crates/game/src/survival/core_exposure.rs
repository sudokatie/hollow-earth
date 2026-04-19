//! Core exposure and radiation sickness system.
//!
//! Tracks cumulative radiation exposure from the hollow sphere's core
//! and applies progressive sickness effects.

/// Stages of core radiation exposure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreExposureStage {
    /// No harmful exposure (0.0-0.2).
    Safe,
    /// Mild exposure with minor effects (0.2-0.5).
    Caution,
    /// Significant exposure with noticeable effects (0.5-0.8).
    Danger,
    /// Severe exposure with major impairment (0.8-0.95).
    Critical,
    /// Fatal exposure level (> 0.95).
    Lethal,
}

/// Effects of radiation sickness.
#[derive(Debug, Clone, Copy)]
pub struct SicknessEffects {
    /// Movement speed reduction (0.0 = no penalty, 1.0 = immobile).
    pub speed_penalty: f32,
    /// Health regeneration reduction (0.0 = normal regen, 1.0 = no regen).
    pub regen_penalty: f32,
    /// Vision impairment (0.0 = clear, 1.0 = blind).
    pub vision_impairment: f32,
}

impl Default for SicknessEffects {
    fn default() -> Self {
        Self {
            speed_penalty: 0.0,
            regen_penalty: 0.0,
            vision_impairment: 0.0,
        }
    }
}

/// Tracks a player's cumulative core radiation exposure.
#[derive(Debug, Clone)]
pub struct CoreExposure {
    /// Total accumulated exposure from 0.0 to 1.0+.
    pub total_exposure: f32,
    /// Current exposure stage based on total exposure.
    pub current_stage: CoreExposureStage,
    /// Current radiation sickness level (0.0-1.0).
    pub sickness_level: f32,
}

impl Default for CoreExposure {
    fn default() -> Self {
        Self {
            total_exposure: 0.0,
            current_stage: CoreExposureStage::Safe,
            sickness_level: 0.0,
        }
    }
}

impl CoreExposure {
    /// Create a new core exposure tracker.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update exposure with radiation over time.
    pub fn update(&mut self, radiation_level: f32, exposure_time: f32) {
        self.total_exposure += calculate_exposure(radiation_level, exposure_time);
        self.current_stage = exposure_stage(self.total_exposure);
        self.sickness_level = self.total_exposure.clamp(0.0, 1.0);
    }

    /// Recover exposure over time in safe zone.
    pub fn recover(&mut self, delta: f32, in_safe_zone: bool) {
        self.total_exposure = recover_exposure(self.total_exposure, delta, in_safe_zone);
        self.current_stage = exposure_stage(self.total_exposure);
        self.sickness_level = self.total_exposure.clamp(0.0, 1.0);
    }

    /// Get current sickness effects.
    #[must_use]
    pub fn effects(&self) -> SicknessEffects {
        radiation_sickness(self.sickness_level)
    }
}

/// Calculate cumulative exposure from radiation over time.
///
/// Exposure accumulates linearly with radiation level and time.
#[must_use]
pub fn calculate_exposure(radiation_level: f32, exposure_time: f32) -> f32 {
    (radiation_level * exposure_time).max(0.0)
}

/// Determine exposure stage from total exposure level.
#[must_use]
pub fn exposure_stage(exposure: f32) -> CoreExposureStage {
    if exposure <= 0.2 {
        CoreExposureStage::Safe
    } else if exposure <= 0.5 {
        CoreExposureStage::Caution
    } else if exposure <= 0.8 {
        CoreExposureStage::Danger
    } else if exposure <= 0.95 {
        CoreExposureStage::Critical
    } else {
        CoreExposureStage::Lethal
    }
}

/// Calculate sickness effects from sickness level.
///
/// Effects scale with sickness level:
/// - Speed penalty: up to 50% reduction at max sickness
/// - Regen penalty: up to 100% reduction at max sickness
/// - Vision impairment: up to 80% impairment at max sickness
#[must_use]
pub fn radiation_sickness(sickness_level: f32) -> SicknessEffects {
    let level = sickness_level.clamp(0.0, 1.0);

    SicknessEffects {
        speed_penalty: level * 0.5,
        regen_penalty: level,
        vision_impairment: level * 0.8,
    }
}

/// Recover exposure over time when in a safe zone.
///
/// Recovery rate is faster in safe zones.
/// Base recovery: 0.01 per second
/// Safe zone bonus: 5x recovery rate
#[must_use]
pub fn recover_exposure(exposure: f32, delta: f32, in_safe_zone: bool) -> f32 {
    let base_recovery = 0.01;
    let recovery_rate = if in_safe_zone {
        base_recovery * 5.0
    } else {
        base_recovery
    };

    (exposure - recovery_rate * delta).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_exposure_basic() {
        let exposure = calculate_exposure(0.5, 2.0);
        assert!((exposure - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_exposure_zero_radiation() {
        let exposure = calculate_exposure(0.0, 10.0);
        assert!((exposure - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_exposure_stage_safe() {
        assert_eq!(exposure_stage(0.0), CoreExposureStage::Safe);
        assert_eq!(exposure_stage(0.2), CoreExposureStage::Safe);
    }

    #[test]
    fn test_exposure_stage_caution() {
        assert_eq!(exposure_stage(0.21), CoreExposureStage::Caution);
        assert_eq!(exposure_stage(0.5), CoreExposureStage::Caution);
    }

    #[test]
    fn test_exposure_stage_danger() {
        assert_eq!(exposure_stage(0.51), CoreExposureStage::Danger);
        assert_eq!(exposure_stage(0.8), CoreExposureStage::Danger);
    }

    #[test]
    fn test_exposure_stage_critical() {
        assert_eq!(exposure_stage(0.81), CoreExposureStage::Critical);
        assert_eq!(exposure_stage(0.95), CoreExposureStage::Critical);
    }

    #[test]
    fn test_exposure_stage_lethal() {
        assert_eq!(exposure_stage(0.96), CoreExposureStage::Lethal);
        assert_eq!(exposure_stage(1.5), CoreExposureStage::Lethal);
    }

    #[test]
    fn test_radiation_sickness_effects() {
        let effects = radiation_sickness(1.0);
        assert!((effects.speed_penalty - 0.5).abs() < 0.001);
        assert!((effects.regen_penalty - 1.0).abs() < 0.001);
        assert!((effects.vision_impairment - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_radiation_sickness_no_sickness() {
        let effects = radiation_sickness(0.0);
        assert!((effects.speed_penalty - 0.0).abs() < 0.001);
        assert!((effects.regen_penalty - 0.0).abs() < 0.001);
        assert!((effects.vision_impairment - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_recover_exposure_safe_zone() {
        let recovered = recover_exposure(0.5, 1.0, true);
        // 0.5 - (0.01 * 5 * 1.0) = 0.45
        assert!((recovered - 0.45).abs() < 0.001);
    }

    #[test]
    fn test_recover_exposure_outside_safe_zone() {
        let recovered = recover_exposure(0.5, 1.0, false);
        // 0.5 - (0.01 * 1.0) = 0.49
        assert!((recovered - 0.49).abs() < 0.001);
    }
}
