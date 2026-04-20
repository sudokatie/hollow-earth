//! Hollow Earth-specific multiplayer synchronization.
//!
//! Handles gravity sync, radiation sync, core pulse sync, tether linking,
//! and spherical coordinate positioning for cooperative sphere exploration.

use glam::Vec3;
use serde::{Deserialize, Serialize};

/// Explorer state for network synchronization.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExplorerState {
    /// Player ID.
    pub player_id: u64,
    /// 3D position on sphere surface.
    pub position: Vec3,
    /// Local gravity direction (normalized, points toward sphere surface).
    pub gravity_direction: Vec3,
    /// Current radiation level (0-100).
    pub radiation: f32,
    /// Whether tether is deployed.
    pub tether_deployed: bool,
    /// ID of tethered partner (if any).
    pub tethered_to: Option<u64>,
    /// Whether player is disoriented (in freefall).
    pub disoriented: bool,
}

impl ExplorerState {
    /// Create a new explorer state.
    #[must_use]
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            position: Vec3::ZERO,
            gravity_direction: Vec3::NEG_Y,
            radiation: 0.0,
            tether_deployed: false,
            tethered_to: None,
            disoriented: false,
        }
    }

    /// Calculate "down" direction for this explorer (gravity pulls toward sphere center).
    /// On a hollow sphere, gravity points outward (away from center).
    #[must_use]
    pub fn local_down(&self) -> Vec3 {
        self.gravity_direction
    }

    /// Check if explorer is in a dangerous radiation zone.
    #[must_use]
    pub fn radiation_warning(&self) -> RadiationWarning {
        if self.radiation < 25.0 {
            RadiationWarning::Safe
        } else if self.radiation < 50.0 {
            RadiationWarning::Caution
        } else if self.radiation < 75.0 {
            RadiationWarning::Danger
        } else {
            RadiationWarning::Lethal
        }
    }
}

/// Radiation warning level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadiationWarning {
    /// Safe (< 25 rads).
    Safe,
    /// Caution (25-50 rads).
    Caution,
    /// Danger (50-75 rads).
    Danger,
    /// Lethal (> 75 rads).
    Lethal,
}

/// Core pulse state synchronized across all clients.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorePulseState {
    /// Current brightness (0.0 to 1.0, 1.0 = full day).
    pub brightness: f32,
    /// Whether a core storm is active.
    pub storm_active: bool,
    /// Storm intensity (0.0 to 2.0, > 1.0 means brighter than normal).
    pub storm_intensity: f32,
}

impl Default for CorePulseState {
    fn default() -> Self {
        Self {
            brightness: 1.0,
            storm_active: false,
            storm_intensity: 0.0,
        }
    }
}

/// Tether link event between players.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TetherLinkEvent {
    /// Player initiating the tether.
    pub initiator_id: u64,
    /// Player being tethered to.
    pub target_id: u64,
    /// Whether link is being established (true) or broken (false).
    pub linked: bool,
}

impl TetherLinkEvent {
    /// Create a new tether link event.
    #[must_use]
    pub fn new(initiator_id: u64, target_id: u64, linked: bool) -> Self {
        Self { initiator_id, target_id, linked }
    }
}

/// Signal flare state for visibility across sphere.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignalFlare {
    /// Position of the flare.
    pub position: Vec3,
    /// Player who launched the flare.
    pub owner_id: u64,
    /// Remaining duration in seconds.
    pub duration: f32,
    /// Color of the flare (as RGB u8 tuple).
    pub color: (u8, u8, u8),
}

impl SignalFlare {
    /// Create a new signal flare.
    #[must_use]
    pub fn new(position: Vec3, owner_id: u64, color: (u8, u8, u8)) -> Self {
        Self {
            position,
            owner_id,
            duration: 30.0,
            color,
        }
    }

    /// Check if the flare is still active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.duration > 0.0
    }
}

/// Manages sphere exploration sync state.
#[derive(Clone, Debug, Default)]
pub struct SphereSync {
    /// Known explorer states.
    explorers: std::collections::HashMap<u64, ExplorerState>,
    /// Current core pulse state.
    core_pulse: CorePulseState,
    /// Active signal flares.
    flares: Vec<SignalFlare>,
    /// Maximum tether distance in meters.
    max_tether_distance: f32,
}

impl SphereSync {
    /// Create a new sphere sync manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            explorers: std::collections::HashMap::new(),
            core_pulse: CorePulseState::default(),
            flares: Vec::new(),
            max_tether_distance: 20.0,
        }
    }

    /// Register an explorer.
    pub fn register(&mut self, state: ExplorerState) {
        self.explorers.insert(state.player_id, state);
    }

    /// Remove an explorer.
    pub fn remove(&mut self, player_id: u64) -> bool {
        // Also break any tethers
        if let Some(explorer) = self.explorers.get_mut(&player_id) {
            explorer.tether_deployed = false;
            explorer.tethered_to = None;
        }
        self.explorers.remove(&player_id).is_some()
    }

    /// Update an explorer's state.
    pub fn update_explorer(&mut self, state: ExplorerState) {
        self.explorers.insert(state.player_id, state);
    }

    /// Update core pulse state.
    pub fn update_core_pulse(&mut self, pulse: CorePulseState) {
        self.core_pulse = pulse;
    }

    /// Get core pulse state.
    #[must_use]
    pub fn core_pulse(&self) -> &CorePulseState {
        &self.core_pulse
    }

    /// Establish a tether link between two explorers.
    pub fn link_tether(&mut self, a_id: u64, b_id: u64) -> bool {
        let a_pos = self.explorers.get(&a_id).map(|e| e.position);
        let b_pos = self.explorers.get(&b_id).map(|e| e.position);

        if let (Some(a_pos), Some(b_pos)) = (a_pos, b_pos) {
            if a_pos.distance(b_pos) <= self.max_tether_distance {
                if let Some(a) = self.explorers.get_mut(&a_id) {
                    a.tether_deployed = true;
                    a.tethered_to = Some(b_id);
                }
                if let Some(b) = self.explorers.get_mut(&b_id) {
                    b.tether_deployed = true;
                    b.tethered_to = Some(a_id);
                }
                return true;
            }
        }
        false
    }

    /// Break a tether link.
    pub fn break_tether(&mut self, player_id: u64) {
        if let Some(explorer) = self.explorers.get(&player_id) {
            let partner_id = explorer.tethered_to;
            if let Some(pid) = partner_id {
                if let Some(partner) = self.explorers.get_mut(&pid) {
                    partner.tether_deployed = false;
                    partner.tethered_to = None;
                }
            }
        }
        if let Some(explorer) = self.explorers.get_mut(&player_id) {
            explorer.tether_deployed = false;
            explorer.tethered_to = None;
        }
    }

    /// Launch a signal flare.
    pub fn launch_flare(&mut self, flare: SignalFlare) {
        self.flares.push(flare);
    }

    /// Update flare durations and remove expired ones.
    pub fn update_flares(&mut self, dt: f32) {
        for flare in &mut self.flares {
            flare.duration -= dt;
        }
        self.flares.retain(|f| f.is_active());
    }

    /// Get active flares.
    #[must_use]
    pub fn active_flares(&self) -> &[SignalFlare] {
        &self.flares
    }

    /// Get explorers needing help (high radiation or disoriented).
    #[must_use]
    pub fn explorers_needing_help(&self) -> Vec<&ExplorerState> {
        self.explorers.values()
            .filter(|e| e.radiation > 50.0 || e.disoriented)
            .collect()
    }

    /// Serialize explorer state for network transmission.
    #[must_use]
    pub fn serialize_explorer(state: &ExplorerState) -> Vec<u8> {
        bincode::serialize(state).unwrap_or_default()
    }

    /// Deserialize explorer state from network data.
    #[must_use]
    pub fn deserialize_explorer(data: &[u8]) -> Option<ExplorerState> {
        bincode::deserialize(data).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explorer_state_new() {
        let state = ExplorerState::new(1);
        assert_eq!(state.player_id, 1);
        assert!((state.radiation - 0.0).abs() < f32::EPSILON);
        assert!(!state.tether_deployed);
    }

    #[test]
    fn test_radiation_warning_safe() {
        let state = ExplorerState { radiation: 10.0, ..ExplorerState::new(1) };
        assert_eq!(state.radiation_warning(), RadiationWarning::Safe);
    }

    #[test]
    fn test_radiation_warning_caution() {
        let state = ExplorerState { radiation: 35.0, ..ExplorerState::new(1) };
        assert_eq!(state.radiation_warning(), RadiationWarning::Caution);
    }

    #[test]
    fn test_radiation_warning_danger() {
        let state = ExplorerState { radiation: 60.0, ..ExplorerState::new(1) };
        assert_eq!(state.radiation_warning(), RadiationWarning::Danger);
    }

    #[test]
    fn test_radiation_warning_lethal() {
        let state = ExplorerState { radiation: 80.0, ..ExplorerState::new(1) };
        assert_eq!(state.radiation_warning(), RadiationWarning::Lethal);
    }

    #[test]
    fn test_core_pulse_default() {
        let pulse = CorePulseState::default();
        assert!((pulse.brightness - 1.0).abs() < f32::EPSILON);
        assert!(!pulse.storm_active);
    }

    #[test]
    fn test_tether_link_event() {
        let event = TetherLinkEvent::new(1, 2, true);
        assert_eq!(event.initiator_id, 1);
        assert!(event.linked);
    }

    #[test]
    fn test_signal_flare_new() {
        let flare = SignalFlare::new(Vec3::ZERO, 1, (255, 0, 0));
        assert!(flare.is_active());
        assert_eq!(flare.color, (255, 0, 0));
    }

    #[test]
    fn test_signal_flare_expired() {
        let mut flare = SignalFlare::new(Vec3::ZERO, 1, (0, 255, 0));
        flare.duration = 0.0;
        assert!(!flare.is_active());
    }

    #[test]
    fn test_sphere_sync_register() {
        let mut sync = SphereSync::new();
        sync.register(ExplorerState::new(1));
        assert!(sync.explorers_needing_help().is_empty());
    }

    #[test]
    fn test_sphere_sync_link_tether() {
        let mut sync = SphereSync::new();
        let mut a = ExplorerState::new(1);
        a.position = Vec3::new(0.0, 0.0, 0.0);
        let mut b = ExplorerState::new(2);
        b.position = Vec3::new(5.0, 0.0, 0.0);
        sync.register(a);
        sync.register(b);

        assert!(sync.link_tether(1, 2));
        assert_eq!(sync.explorers.get(&1).unwrap().tethered_to, Some(2));
        assert_eq!(sync.explorers.get(&2).unwrap().tethered_to, Some(1));
    }

    #[test]
    fn test_sphere_sync_link_tether_too_far() {
        let mut sync = SphereSync::new();
        let mut a = ExplorerState::new(1);
        a.position = Vec3::new(0.0, 0.0, 0.0);
        let mut b = ExplorerState::new(2);
        b.position = Vec3::new(50.0, 0.0, 0.0);
        sync.register(a);
        sync.register(b);

        assert!(!sync.link_tether(1, 2));
    }

    #[test]
    fn test_sphere_sync_break_tether() {
        let mut sync = SphereSync::new();
        let mut a = ExplorerState::new(1);
        a.position = Vec3::new(0.0, 0.0, 0.0);
        let mut b = ExplorerState::new(2);
        b.position = Vec3::new(5.0, 0.0, 0.0);
        sync.register(a);
        sync.register(b);

        sync.link_tether(1, 2);
        sync.break_tether(1);

        assert!(sync.explorers.get(&1).unwrap().tethered_to.is_none());
        assert!(sync.explorers.get(&2).unwrap().tethered_to.is_none());
    }

    #[test]
    fn test_sphere_sync_flares() {
        let mut sync = SphereSync::new();
        sync.launch_flare(SignalFlare::new(Vec3::ZERO, 1, (255, 0, 0)));
        assert_eq!(sync.active_flares().len(), 1);

        sync.update_flares(35.0);
        assert!(sync.active_flares().is_empty());
    }

    #[test]
    fn test_sphere_sync_explorers_needing_help() {
        let mut sync = SphereSync::new();
        sync.register(ExplorerState::new(1));

        let mut irradiated = ExplorerState::new(2);
        irradiated.radiation = 60.0;
        sync.register(irradiated);

        assert_eq!(sync.explorers_needing_help().len(), 1);
    }

    #[test]
    fn test_sphere_sync_core_pulse_update() {
        let mut sync = SphereSync::new();
        let storm = CorePulseState { brightness: 2.0, storm_active: true, storm_intensity: 1.5 };
        sync.update_core_pulse(storm);
        assert!(sync.core_pulse().storm_active);
    }

    #[test]
    fn test_sphere_sync_serialize_deserialize() {
        let state = ExplorerState::new(42);
        let data = SphereSync::serialize_explorer(&state);
        let restored = SphereSync::deserialize_explorer(&data).unwrap();
        assert_eq!(restored.player_id, 42);
    }
}
