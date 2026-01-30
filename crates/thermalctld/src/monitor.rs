use anyhow::Result;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, info, warn};

use crate::chassis::Chassis;
use crate::updaters::{FanUpdater, TemperatureUpdater};

pub struct ThermalMonitor {
    chassis: Chassis,
    fan_updater: FanUpdater,
    temperature_updater: TemperatureUpdater,
    initial_interval: Duration,
    update_interval: Duration,
    update_elapsed_threshold: Duration,
    wait_time: Duration,
}

impl ThermalMonitor {
    pub async fn new(
        chassis: Chassis,
        initial_interval_secs: u64,
        update_interval_secs: u64,
        update_elapsed_threshold_secs: u64,
    ) -> Result<Self> {
        let initial_interval = Duration::from_secs(initial_interval_secs);
        let update_interval = Duration::from_secs(update_interval_secs);
        let update_elapsed_threshold = Duration::from_secs(update_elapsed_threshold_secs);

        Ok(Self {
            chassis: chassis.clone(),
            fan_updater: FanUpdater::new(chassis.clone()).await?,
            temperature_updater: TemperatureUpdater::new(chassis).await?,
            initial_interval,
            update_interval,
            update_elapsed_threshold,
            wait_time: initial_interval,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Starting thermal monitoring loop");

        loop {
            sleep(self.wait_time).await;

            let start = Instant::now();
            self.update().await?;
            let elapsed = start.elapsed();

            if elapsed < self.update_interval {
                self.wait_time = self.update_interval - elapsed;
            } else {
                self.wait_time = self.initial_interval;
            }

            if elapsed > self.update_elapsed_threshold {
                warn!(
                    "Update fan and temperature status took {:?}, there might be performance risk",
                    elapsed
                );
            }

            debug!("Thermal monitor cycle completed in {:?}", elapsed);
        }
    }

    async fn update(&self) -> Result<()> {
        debug!("Updating thermal status");

        if let Err(e) = self.fan_updater.update().await {
            warn!("Failed to update fan status: {}", e);
        }

        if let Err(e) = self.temperature_updater.update().await {
            warn!("Failed to update temperature status: {}", e);
        }

        Ok(())
    }
}
