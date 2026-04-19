//! Crafting system with recipes and execution.

mod executor;
mod furnace;
mod registry;
pub mod sphere_stations;
pub mod sphere_weapons;

pub use executor::{check_craft, execute_craft, execute_craft_by_id, CraftError, CraftRequirements};
pub use furnace::{
    Furnace, FurnaceState, FuelEntry, DEFAULT_SMELT_TIME, FUEL_CHARCOAL, FUEL_COAL,
    FUEL_LAVA_BUCKET, FUEL_STICK, FUEL_WOOD,
};
pub use registry::{CraftingStation, Ingredient, Recipe, RecipeRegistry};
pub use sphere_stations::{
    CoreKiln, CrystalForge, FungalWorkbench, MaterialType, SpinningVat, SphereStationType,
    StationRequirements,
};
pub use sphere_weapons::{
    ChitinSpear, CoreSiphon, CrystalPickaxe, GravityHammer, LightStaff, SphereWeaponEffect,
    SporeGrenade,
};
