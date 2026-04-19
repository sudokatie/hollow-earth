//! In-game HUD elements.

mod crosshair;
mod debug_console;
mod debug_overlay;
mod depth_indicator;
mod gravity_indicator;
mod health_bar;
mod hotbar;
mod hunger_bar;
mod radiation_indicator;
mod status_effects;
mod tether_status;
mod tooltip;

pub use crosshair::{CrosshairConfig, CrosshairStyle, draw_crosshair};
pub use debug_console::{
    process_builtin_command, ConsoleAction, ConsoleLine, DebugConsole, LineKind,
};
pub use debug_overlay::{DebugLevel, DebugOverlay, DebugStats};
pub use depth_indicator::{
    calculate_depth_zone, draw_depth_indicator, format_altitude, DepthIndicator, DepthZone,
};
pub use gravity_indicator::{
    draw_gravity_indicator, format_direction, GravityIndicator,
};
pub use health_bar::{draw_health_bar, HealthBarState};
pub use hotbar::{draw_hotbar, HotbarSlot, ItemTextures};
pub use hunger_bar::{draw_hunger_bar, HungerBarState};
pub use radiation_indicator::{
    draw_radiation_indicator, format_radiation, warning_from_level, RadiationIndicator,
    RadiationWarning,
};
pub use status_effects::{
    ActiveStatusEffect, StatusEffectKind, ICON_SIZE, draw_status_effects,
};
pub use tether_status::{
    draw_tether_status, format_length, TetherHealth, TetherInfo, TetherStatus,
};
pub use tooltip::{ItemTooltip, draw_tooltip};
