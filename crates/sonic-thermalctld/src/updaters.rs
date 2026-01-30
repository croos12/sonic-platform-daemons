use anyhow::{Context, Result};
use chrono::Local;
use std::collections::HashMap;
use std::sync::Arc;
use swss_common::{CxxString, DbConnector, Table};
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

use crate::chassis::Chassis;
use sonic_platform::{Fan, FanDrawer, FanStatus, LedColor, Thermal, TemperatureStatus};

const FAN_INFO_TABLE: &str = "FAN_INFO";
const FAN_DRAWER_INFO_TABLE: &str = "FAN_DRAWER_INFO";
const TEMPERATURE_INFO_TABLE: &str = "TEMPERATURE_INFO";
const PHYSICAL_ENTITY_INFO_TABLE: &str = "PHYSICAL_ENTITY_INFO";
const NOT_AVAILABLE: &str = "N/A";

pub struct FanUpdater {
    chassis: Chassis,
    fan_status_dict: Arc<Mutex<HashMap<String, FanStatus>>>,
    table: Table,
    drawer_table: Table,
    phy_entity_table: Table,
}

impl FanUpdater {
    pub async fn new(chassis: Chassis) -> Result<Self> {
        let state_db = DbConnector::new_named("STATE_DB", false, 0)
            .context("Failed to connect to STATE_DB")?;
        let state_db2 = DbConnector::new_named("STATE_DB", false, 0)
            .context("Failed to connect to STATE_DB")?;
        let state_db3 = DbConnector::new_named("STATE_DB", false, 0)
            .context("Failed to connect to STATE_DB")?;

        let table = Table::new(state_db, FAN_INFO_TABLE)
            .context("Failed to create FAN_INFO table")?;
        let drawer_table = Table::new(state_db2, FAN_DRAWER_INFO_TABLE)
            .context("Failed to create FAN_DRAWER_INFO table")?;
        let phy_entity_table = Table::new(state_db3, PHYSICAL_ENTITY_INFO_TABLE)
            .context("Failed to create PHYSICAL_ENTITY_INFO table")?;

        Ok(Self {
            chassis,
            fan_status_dict: Arc::new(Mutex::new(HashMap::new())),
            table,
            drawer_table,
            phy_entity_table,
        })
    }

    pub async fn update(&self) -> Result<()> {
        debug!("Starting fan update");

        for (drawer_index, drawer) in self.chassis.get_all_fan_drawers().iter().enumerate() {
            self.update_fan_drawer(drawer, drawer_index).await?;

            for (fan_index, fan) in drawer.get_all_fans().iter().enumerate() {
                if let Err(e) = self.update_fan(drawer, drawer_index, fan.as_ref(), fan_index).await {
                    warn!("Failed to update fan status: {}", e);
                }
            }
        }

        debug!("Finished fan update");
        Ok(())
    }

    async fn update_fan_drawer(&self, drawer: &FanDrawer, _index: usize) -> Result<()> {
        let name = drawer.get_name()?;

        let fvs = vec![
            ("presence", drawer.get_presence()?.to_string()),
            ("model", drawer.get_model().unwrap_or_else(|_| NOT_AVAILABLE.to_string())),
            ("serial", drawer.get_serial().unwrap_or_else(|_| NOT_AVAILABLE.to_string())),
            ("status", drawer.get_status()?.to_string()),
            ("is_replaceable", drawer.is_replaceable()?.to_string()),
        ];

        self.drawer_table.set(&name, fvs)?;
        Ok(())
    }

    async fn update_fan(&self, drawer: &FanDrawer, _drawer_index: usize,
                        fan: &dyn Fan, _fan_index: usize) -> Result<()> {
        let fan_name = fan.get_name()?;
        let presence = fan.get_presence().unwrap_or(false);

        let timestamp = Local::now().format("%Y%m%d %H:%M:%S").to_string();

        if presence {
            let drawer_name = drawer.get_name()?;
            let model = fan.get_model().unwrap_or_else(|_| NOT_AVAILABLE.to_string());
            let serial = fan.get_serial().unwrap_or_else(|_| NOT_AVAILABLE.to_string());
            let status = fan.get_status()?.to_string();
            let direction = fan.get_direction()?.to_string();
            let speed = fan.get_speed()?.to_string();
            let speed_target = fan.get_target_speed()?.to_string();
            let is_under_speed = fan.is_under_speed()?.to_string();
            let is_over_speed = fan.is_over_speed()?.to_string();
            let is_replaceable = fan.is_replaceable()?.to_string();

            let fvs = vec![
                ("presence", "true"),
                ("drawer_name", drawer_name.as_str()),
                ("model", model.as_str()),
                ("serial", serial.as_str()),
                ("status", status.as_str()),
                ("direction", direction.as_str()),
                ("speed", speed.as_str()),
                ("speed_target", speed_target.as_str()),
                ("is_under_speed", is_under_speed.as_str()),
                ("is_over_speed", is_over_speed.as_str()),
                ("is_replaceable", is_replaceable.as_str()),
                ("timestamp", timestamp.as_str()),
            ];
            self.table.set(&fan_name, fvs)?;
        } else {
            let fvs = vec![
                ("presence", "false"),
                ("status", "false"),
                ("timestamp", timestamp.as_str()),
            ];
            self.table.set(&fan_name, fvs)?;
        }

        Ok(())
    }
}

pub struct TemperatureUpdater {
    chassis: Chassis,
    temperature_status_dict: Arc<Mutex<HashMap<String, TemperatureStatus>>>,
    table: Table,
    phy_entity_table: Table,
    chassis_table: Option<Table>,
}

impl TemperatureUpdater {
    pub async fn new(chassis: Chassis) -> Result<Self> {
        let state_db = DbConnector::new_named("STATE_DB", false, 0)
            .context("Failed to connect to STATE_DB")?;
        let state_db2 = DbConnector::new_named("STATE_DB", false, 0)
            .context("Failed to connect to STATE_DB")?;

        let table = Table::new(state_db, TEMPERATURE_INFO_TABLE)
            .context("Failed to create TEMPERATURE_INFO table")?;
        let phy_entity_table = Table::new(state_db2, PHYSICAL_ENTITY_INFO_TABLE)
            .context("Failed to create PHYSICAL_ENTITY_INFO table")?;

        let chassis_table = None;

        Ok(Self {
            chassis,
            temperature_status_dict: Arc::new(Mutex::new(HashMap::new())),
            table,
            phy_entity_table,
            chassis_table,
        })
    }

    pub async fn update(&self) -> Result<()> {
        debug!("Starting temperature update");

        for (index, thermal) in self.chassis.get_all_thermals().iter().enumerate() {
            if let Err(e) = self.update_thermal(thermal.as_ref(), index).await {
                warn!("Failed to update thermal status: {}", e);
            }
        }

        debug!("Finished temperature update");
        Ok(())
    }

    async fn update_thermal(&self, thermal: &dyn Thermal, _index: usize) -> Result<()> {
        let name = thermal.get_name()?;
        let temperature = thermal.get_temperature().unwrap_or(-999.0);
        let timestamp = Local::now().format("%Y%m%d %H:%M:%S").to_string();

        let high_threshold = thermal.get_high_threshold().unwrap_or(-999.0);
        let low_threshold = thermal.get_low_threshold().unwrap_or(-999.0);
        let warning_status = if temperature > high_threshold || temperature < low_threshold {
            "True"
        } else {
            "False"
        };

        let fvs = vec![
            ("temperature", temperature.to_string()),
            ("minimum_temperature", thermal.get_minimum_recorded().unwrap_or(-999.0).to_string()),
            ("maximum_temperature", thermal.get_maximum_recorded().unwrap_or(-999.0).to_string()),
            ("high_threshold", high_threshold.to_string()),
            ("low_threshold", low_threshold.to_string()),
            ("warning_status", warning_status.to_string()),
            ("critical_high_threshold", thermal.get_high_critical_threshold().unwrap_or(-999.0).to_string()),
            ("critical_low_threshold", thermal.get_low_critical_threshold().unwrap_or(-999.0).to_string()),
            ("is_replaceable", thermal.is_replaceable().unwrap_or(false).to_string()),
            ("timestamp", timestamp),
        ];

        self.table.set(&name, fvs)?;
        Ok(())
    }
}
