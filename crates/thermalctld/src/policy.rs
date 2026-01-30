use anyhow::Result;
use std::path::Path;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, info, warn};

use crate::chassis::Chassis;

const POLICY_FILE: &str = "/usr/share/sonic/platform/thermal_policy.json";
const DEFAULT_INTERVAL_SECS: u64 = 60;
const RUN_POLICY_WARN_THRESHOLD_SECS: u64 = 30;
const FAST_START_INTERVAL_SECS: u64 = 15;

pub struct ThermalPolicyManager {
    chassis: Chassis,
    interval: Duration,
    policy_loaded: bool,
}

impl ThermalPolicyManager {
    pub fn new(chassis: Chassis) -> Result<Self> {
        let policy_loaded = Path::new(POLICY_FILE).exists();

        if !policy_loaded {
            info!("Thermal policy file not found at {}, thermal control policy disabled", POLICY_FILE);
        }

        Ok(Self {
            chassis,
            interval: Duration::from_secs(DEFAULT_INTERVAL_SECS),
            policy_loaded,
        })
    }

    pub fn load_policy(&mut self) -> Result<()> {
        if !Path::new(POLICY_FILE).exists() {
            return Ok(());
        }

        info!("Loading thermal policy from {}", POLICY_FILE);
        self.policy_loaded = true;
        Ok(())
    }

    pub fn init_algorithm(&self) -> Result<()> {
        if !self.policy_loaded {
            return Ok(());
        }

        info!("Initializing thermal control algorithm");
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        if !self.policy_loaded {
            info!("Thermal policy not loaded, skipping policy execution");
            return Ok(());
        }

        info!("Starting thermal policy loop");

        self.load_policy()?;
        self.init_algorithm()?;

        let mut wait_time = self.interval;

        loop {
            sleep(wait_time).await;

            let start = Instant::now();
            if let Err(e) = self.run_policy().await {
                warn!("Failed to execute thermal policy: {}", e);
            }
            let elapsed = start.elapsed();

            if elapsed < self.interval {
                wait_time = self.interval - elapsed;
            } else {
                wait_time = Duration::from_secs(FAST_START_INTERVAL_SECS);
            }

            if elapsed > Duration::from_secs(RUN_POLICY_WARN_THRESHOLD_SECS) {
                warn!(
                    "Thermal policy execution took {:?}, there might be performance risk",
                    elapsed
                );
            }

            debug!("Thermal policy cycle completed in {:?}", elapsed);
        }
    }

    async fn run_policy(&self) -> Result<()> {
        if !self.policy_loaded {
            return Ok(());
        }

        debug!("Executing thermal policy");

        Ok(())
    }

    pub fn deinitialize(&self) -> Result<()> {
        if self.policy_loaded {
            info!("Deinitializing thermal policy");
        }
        Ok(())
    }
}

impl Drop for ThermalPolicyManager {
    fn drop(&mut self) {
        if let Err(e) = self.deinitialize() {
            warn!("Failed to deinitialize thermal policy: {}", e);
        }
    }
}
