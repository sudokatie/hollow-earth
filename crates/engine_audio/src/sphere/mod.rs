//! Sphere-specific audio systems for hollow world acoustics.

pub mod echo;

pub use echo::{
    calculate_echo, core_hum, crystal_resonance, echo_base_delay, CoreHum, ResonanceEffect,
    SphereEcho, CORE_HUM_FREQUENCY, ECHO_SPEED_OF_SOUND,
};
