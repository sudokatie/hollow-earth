//! Survival mechanics: health, hunger, and status effects.

mod combat;
mod core_exposure;
mod death;
mod health;
mod hunger;
mod mining;

pub use combat::{
    attempt_attack, calculate_knockback, can_attack, AttackCooldown, AttackResult, CombatStats,
};
pub use death::{
    DeathCause, DeathHandler, DeathResult, DroppedItem, ITEM_DESPAWN_TIME, PICKUP_DELAY_SECS,
};
pub use health::{DamageSource, Health};
pub use hunger::Hunger;
pub use mining::{
    calculate_mining_time, MiningProgress, MiningResult, BlockPos, BASE_MINE_TIME_SECS,
    BASE_MINING_SPEED,
};
pub use core_exposure::{
    calculate_exposure, exposure_stage, radiation_sickness, recover_exposure,
    CoreExposure, CoreExposureStage, SicknessEffects,
};
