use anyhow::Result;
use clap::Parser;
use swss_common::sonic_db_config_initialize_global;
use tokio::signal;
use tracing::{info, error};

use thermalctld::chassis::Chassis;
use thermalctld::monitor::ThermalMonitor;
use thermalctld::policy::ThermalPolicyManager;

const SYSLOG_IDENTIFIER: &str = "thermalctld";
const DEFAULT_DB_CONFIG_PATH: &str = "/var/run/redis/sonic-db/database_config.json";

#[derive(Parser, Debug)]
#[command(name = "thermalctld", about = "Thermal control daemon for SONiC")]
struct Args {
    #[arg(long, default_value = "5")]
    thermal_monitor_initial_interval: u64,

    #[arg(long, default_value = "60")]
    thermal_monitor_update_interval: u64,

    #[arg(long, default_value = "30")]
    thermal_monitor_update_elapsed_threshold: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt::init();

    info!("Starting thermal control daemon...");

    sonic_db_config_initialize_global(DEFAULT_DB_CONFIG_PATH, false)?;

    let chassis = {
        #[cfg(feature = "mellanox")]
        {
            use thermalctld::platforms::mellanox;
            if mellanox::detect_platform() {
                info!("Detected Mellanox platform, loading platform-specific implementation");
                let mlnx_chassis = mellanox::MlnxChassis::new()?;
                let (_fans, fan_drawers, thermals) = mlnx_chassis.into_components();
                Chassis::from_platform_components(fan_drawers, thermals)
            } else {
                Chassis::new()?
            }
        }
        #[cfg(not(feature = "mellanox"))]
        {
            Chassis::new()?
        }
    };
    info!("Chassis initialized successfully");

    let mut monitor = ThermalMonitor::new(
        chassis.clone(),
        args.thermal_monitor_initial_interval,
        args.thermal_monitor_update_interval,
        args.thermal_monitor_update_elapsed_threshold,
    ).await?;

    let mut policy_manager = ThermalPolicyManager::new(chassis.clone())?;

    info!("Thermal control daemon started successfully");

    tokio::select! {
        result = monitor.run() => {
            if let Err(e) = result {
                error!("Thermal monitor error: {}", e);
            }
        }
        result = policy_manager.run() => {
            if let Err(e) = result {
                error!("Thermal policy manager error: {}", e);
            }
        }
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal, stopping...");
        }
    }

    info!("Thermal control daemon stopped");
    Ok(())
}