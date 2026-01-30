use anyhow::Result;
use std::sync::Arc;

use sonic_platform::{Fan, FanDrawer, Thermal};

#[derive(Clone)]
pub struct Chassis {
    fan_drawers: Arc<Vec<FanDrawer>>,
    thermals: Arc<Vec<Box<dyn Thermal>>>,
}

impl Chassis {
    pub fn new() -> Result<Self> {
        Ok(Self {
            fan_drawers: Arc::new(Vec::new()),
            thermals: Arc::new(Vec::new()),
        })
    }

    pub fn from_platform_components(
        fan_drawers: Vec<FanDrawer>,
        thermals: Vec<Box<dyn Thermal>>,
    ) -> Self {
        Self {
            fan_drawers: Arc::new(fan_drawers),
            thermals: Arc::new(thermals),
        }
    }

    pub fn get_all_fan_drawers(&self) -> &[FanDrawer] {
        &self.fan_drawers
    }

    pub fn get_all_thermals(&self) -> &[Box<dyn Thermal>] {
        &self.thermals
    }

    pub fn is_modular_chassis(&self) -> bool {
        false
    }

    pub fn is_smartswitch(&self) -> bool {
        false
    }

    pub fn is_dpu(&self) -> bool {
        false
    }

    pub fn get_my_slot(&self) -> Result<i32> {
        Err(anyhow::anyhow!("Not a modular chassis"))
    }

    pub fn get_dpu_id(&self) -> Result<i32> {
        Err(anyhow::anyhow!("Not a SmartSwitch DPU"))
    }
}
