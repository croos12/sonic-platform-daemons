
use std::sync::RwLock;
use anyhow::Result;
use sonic_platform::{Fan, FanDirection, LedColor, Thermal};

pub const MODULE_INVALID_SLOT: i32 = -1;

pub struct MockFan {
    name: RwLock<Option<String>>,
    presence: RwLock<bool>,
    model: RwLock<String>,
    serial: RwLock<String>,
    status: RwLock<bool>,
    position_in_parent: RwLock<usize>,
    replaceable: RwLock<bool>,
    speed: RwLock<u32>,
    speed_tolerance: RwLock<u32>,
    target_speed: RwLock<u32>,
    direction: RwLock<FanDirection>,
    status_led: RwLock<LedColor>,
}

impl MockFan {
    pub fn new() -> Self {
        Self {
            name: RwLock::new(None),
            presence: RwLock::new(true),
            model: RwLock::new("Fan Model".to_string()),
            serial: RwLock::new("Fan Serial".to_string()),
            status: RwLock::new(true),
            position_in_parent: RwLock::new(1),
            replaceable: RwLock::new(true),
            speed: RwLock::new(20),
            speed_tolerance: RwLock::new(20),
            target_speed: RwLock::new(20),
            direction: RwLock::new(FanDirection::Intake),
            status_led: RwLock::new(LedColor::Red),
        }
    }

    pub fn set_name(&self, name: Option<String>) {
        *self.name.write().unwrap() = name;
    }

    pub fn set_presence(&self, presence: bool) {
        *self.presence.write().unwrap() = presence;
    }

    pub fn set_status(&self, status: bool) {
        *self.status.write().unwrap() = status;
    }

    pub fn set_speed(&self, speed: u32) {
        *self.speed.write().unwrap() = speed;
    }

    pub fn set_target_speed(&self, speed: u32) {
        *self.target_speed.write().unwrap() = speed;
    }

    pub fn set_speed_tolerance(&self, tolerance: u32) {
        *self.speed_tolerance.write().unwrap() = tolerance;
    }

    pub fn set_direction(&self, direction: FanDirection) {
        *self.direction.write().unwrap() = direction;
    }

    pub fn get_speed_tolerance(&self) -> Result<u32> {
        Ok(*self.speed_tolerance.read().unwrap())
    }

    pub fn make_under_speed(&self) {
        *self.speed.write().unwrap() = 1;
        *self.target_speed.write().unwrap() = 2;
        *self.speed_tolerance.write().unwrap() = 0;
    }

    pub fn make_over_speed(&self) {
        *self.speed.write().unwrap() = 2;
        *self.target_speed.write().unwrap() = 1;
        *self.speed_tolerance.write().unwrap() = 0;
    }

    pub fn make_normal_speed(&self) {
        *self.speed.write().unwrap() = 1;
        *self.target_speed.write().unwrap() = 1;
        *self.speed_tolerance.write().unwrap() = 0;
    }
}

impl Default for MockFan {
    fn default() -> Self {
        Self::new()
    }
}

impl Fan for MockFan {
    fn get_name(&self) -> Result<String> {
        Ok(self.name.read().unwrap().clone().unwrap_or_default())
    }

    fn get_presence(&self) -> Result<bool> {
        Ok(*self.presence.read().unwrap())
    }

    fn get_status(&self) -> Result<bool> {
        Ok(*self.status.read().unwrap())
    }

    fn get_speed(&self) -> Result<u32> {
        Ok(*self.speed.read().unwrap())
    }

    fn get_target_speed(&self) -> Result<u32> {
        Ok(*self.target_speed.read().unwrap())
    }

    fn is_under_speed(&self) -> Result<bool> {
        let speed = *self.speed.read().unwrap();
        let target = *self.target_speed.read().unwrap();
        let tolerance = *self.speed_tolerance.read().unwrap();
        Ok(speed < target.saturating_sub(tolerance))
    }

    fn is_over_speed(&self) -> Result<bool> {
        let speed = *self.speed.read().unwrap();
        let target = *self.target_speed.read().unwrap();
        let tolerance = *self.speed_tolerance.read().unwrap();
        Ok(speed > target + tolerance)
    }

    fn get_direction(&self) -> Result<FanDirection> {
        Ok(*self.direction.read().unwrap())
    }

    fn get_model(&self) -> Result<String> {
        Ok(self.model.read().unwrap().clone())
    }

    fn get_serial(&self) -> Result<String> {
        Ok(self.serial.read().unwrap().clone())
    }

    fn is_replaceable(&self) -> Result<bool> {
        Ok(*self.replaceable.read().unwrap())
    }

    fn get_position_in_parent(&self) -> Result<usize> {
        Ok(*self.position_in_parent.read().unwrap())
    }

    fn set_status_led(&self, color: LedColor) -> Result<()> {
        *self.status_led.write().unwrap() = color;
        Ok(())
    }

    fn get_status_led(&self) -> Result<LedColor> {
        Ok(*self.status_led.read().unwrap())
    }
}

pub struct MockErrorFan {
    base: MockFan,
}

impl MockErrorFan {
    pub fn new() -> Self {
        Self {
            base: MockFan::new(),
        }
    }
}

impl Default for MockErrorFan {
    fn default() -> Self {
        Self::new()
    }
}

impl Fan for MockErrorFan {
    fn get_name(&self) -> Result<String> {
        self.base.get_name()
    }

    fn get_presence(&self) -> Result<bool> {
        self.base.get_presence()
    }

    fn get_status(&self) -> Result<bool> {
        self.base.get_status()
    }

    fn get_speed(&self) -> Result<u32> {
        Err(anyhow::anyhow!("Failed to get speed"))
    }

    fn get_target_speed(&self) -> Result<u32> {
        self.base.get_target_speed()
    }

    fn is_under_speed(&self) -> Result<bool> {
        self.base.is_under_speed()
    }

    fn is_over_speed(&self) -> Result<bool> {
        self.base.is_over_speed()
    }

    fn get_direction(&self) -> Result<FanDirection> {
        self.base.get_direction()
    }

    fn get_model(&self) -> Result<String> {
        self.base.get_model()
    }

    fn get_serial(&self) -> Result<String> {
        self.base.get_serial()
    }

    fn is_replaceable(&self) -> Result<bool> {
        self.base.is_replaceable()
    }

    fn get_position_in_parent(&self) -> Result<usize> {
        self.base.get_position_in_parent()
    }

    fn set_status_led(&self, color: LedColor) -> Result<()> {
        self.base.set_status_led(color)
    }

    fn get_status_led(&self) -> Result<LedColor> {
        self.base.get_status_led()
    }
}

pub struct MockFanDrawer {
    name: String,
    presence: RwLock<bool>,
    model: String,
    serial: String,
    status: RwLock<bool>,
    position_in_parent: usize,
    replaceable: bool,
    status_led: RwLock<LedColor>,
    fan_list: RwLock<Vec<Box<dyn Fan>>>,
}

impl MockFanDrawer {
    pub fn new(index: usize) -> Self {
        Self {
            name: format!("FanDrawer {}", index),
            presence: RwLock::new(true),
            model: "Fan Drawer Model".to_string(),
            serial: "Fan Drawer Serial".to_string(),
            status: RwLock::new(true),
            position_in_parent: 1,
            replaceable: true,
            status_led: RwLock::new(LedColor::Red),
            fan_list: RwLock::new(Vec::new()),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_presence(&self) -> bool {
        *self.presence.read().unwrap()
    }

    pub fn set_presence(&self, presence: bool) {
        *self.presence.write().unwrap() = presence;
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }

    pub fn get_serial(&self) -> &str {
        &self.serial
    }

    pub fn get_status(&self) -> bool {
        *self.status.read().unwrap()
    }

    pub fn set_status(&self, status: bool) {
        *self.status.write().unwrap() = status;
    }

    pub fn get_position_in_parent(&self) -> usize {
        self.position_in_parent
    }

    pub fn is_replaceable(&self) -> bool {
        self.replaceable
    }

    pub fn get_status_led(&self) -> LedColor {
        *self.status_led.read().unwrap()
    }

    pub fn set_status_led(&self, color: LedColor) {
        *self.status_led.write().unwrap() = color;
    }

    pub fn get_all_fans(&self) -> std::sync::RwLockReadGuard<Vec<Box<dyn Fan>>> {
        self.fan_list.read().unwrap()
    }

    pub fn add_fan(&self, fan: Box<dyn Fan>) {
        self.fan_list.write().unwrap().push(fan);
    }
}

pub struct MockPsu {
    name: RwLock<Option<String>>,
    presence: RwLock<bool>,
    model: String,
    serial: String,
    status: RwLock<bool>,
    position_in_parent: usize,
    replaceable: bool,
    fan_list: RwLock<Vec<Box<dyn Fan>>>,
    thermal_list: RwLock<Vec<Box<dyn Thermal>>>,
}

impl MockPsu {
    pub fn new() -> Self {
        Self {
            name: RwLock::new(None),
            presence: RwLock::new(true),
            model: "PSU Model".to_string(),
            serial: "PSU Serial".to_string(),
            status: RwLock::new(true),
            position_in_parent: 1,
            replaceable: true,
            fan_list: RwLock::new(Vec::new()),
            thermal_list: RwLock::new(Vec::new()),
        }
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.read().unwrap().clone()
    }

    pub fn get_presence(&self) -> bool {
        *self.presence.read().unwrap()
    }

    pub fn set_presence(&self, presence: bool) {
        *self.presence.write().unwrap() = presence;
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }

    pub fn get_serial(&self) -> &str {
        &self.serial
    }

    pub fn get_status(&self) -> bool {
        *self.status.read().unwrap()
    }

    pub fn get_powergood_status(&self) -> bool {
        *self.status.read().unwrap()
    }

    pub fn set_status(&self, status: bool) {
        *self.status.write().unwrap() = status;
    }

    pub fn get_position_in_parent(&self) -> usize {
        self.position_in_parent
    }

    pub fn is_replaceable(&self) -> bool {
        self.replaceable
    }

    pub fn get_all_fans(&self) -> std::sync::RwLockReadGuard<Vec<Box<dyn Fan>>> {
        self.fan_list.read().unwrap()
    }

    pub fn add_fan(&self, fan: Box<dyn Fan>) {
        self.fan_list.write().unwrap().push(fan);
    }

    pub fn get_all_thermals(&self) -> std::sync::RwLockReadGuard<Vec<Box<dyn Thermal>>> {
        self.thermal_list.read().unwrap()
    }

    pub fn add_thermal(&self, thermal: Box<dyn Thermal>) {
        self.thermal_list.write().unwrap().push(thermal);
    }
}

impl Default for MockPsu {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MockSfp {
    name: RwLock<Option<String>>,
    presence: RwLock<bool>,
    model: String,
    serial: String,
    status: RwLock<bool>,
    position_in_parent: usize,
    replaceable: bool,
    thermal_list: RwLock<Vec<Box<dyn Thermal>>>,
}

impl MockSfp {
    pub fn new() -> Self {
        Self {
            name: RwLock::new(None),
            presence: RwLock::new(true),
            model: "SFP Model".to_string(),
            serial: "SFP Serial".to_string(),
            status: RwLock::new(true),
            position_in_parent: 1,
            replaceable: true,
            thermal_list: RwLock::new(Vec::new()),
        }
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.read().unwrap().clone()
    }

    pub fn get_presence(&self) -> bool {
        *self.presence.read().unwrap()
    }

    pub fn set_presence(&self, presence: bool) {
        *self.presence.write().unwrap() = presence;
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }

    pub fn get_serial(&self) -> &str {
        &self.serial
    }

    pub fn get_status(&self) -> bool {
        *self.status.read().unwrap()
    }

    pub fn set_status(&self, status: bool) {
        *self.status.write().unwrap() = status;
    }

    pub fn get_position_in_parent(&self) -> usize {
        self.position_in_parent
    }

    pub fn is_replaceable(&self) -> bool {
        self.replaceable
    }

    pub fn get_all_thermals(&self) -> std::sync::RwLockReadGuard<Vec<Box<dyn Thermal>>> {
        self.thermal_list.read().unwrap()
    }

    pub fn add_thermal(&self, thermal: Box<dyn Thermal>) {
        self.thermal_list.write().unwrap().push(thermal);
    }
}

impl Default for MockSfp {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MockThermal {
    name: RwLock<Option<String>>,
    presence: RwLock<bool>,
    model: String,
    serial: String,
    status: RwLock<bool>,
    position_in_parent: RwLock<usize>,
    replaceable: RwLock<bool>,
    temperature: RwLock<f32>,
    minimum_temperature: RwLock<f32>,
    maximum_temperature: RwLock<f32>,
    high_threshold: RwLock<f32>,
    low_threshold: RwLock<f32>,
    high_critical_threshold: RwLock<f32>,
    low_critical_threshold: RwLock<f32>,
}

impl MockThermal {
    pub fn new(index: Option<usize>) -> Self {
        Self {
            name: RwLock::new(index.map(|i| format!("Thermal {}", i))),
            presence: RwLock::new(true),
            model: "Thermal Model".to_string(),
            serial: "Thermal Serial".to_string(),
            status: RwLock::new(true),
            position_in_parent: RwLock::new(1),
            replaceable: RwLock::new(false),
            temperature: RwLock::new(2.0),
            minimum_temperature: RwLock::new(1.0),
            maximum_temperature: RwLock::new(5.0),
            high_threshold: RwLock::new(3.0),
            low_threshold: RwLock::new(1.0),
            high_critical_threshold: RwLock::new(4.0),
            low_critical_threshold: RwLock::new(0.0),
        }
    }

    pub fn set_name(&self, name: Option<String>) {
        *self.name.write().unwrap() = name;
    }

    pub fn set_presence(&self, presence: bool) {
        *self.presence.write().unwrap() = presence;
    }

    pub fn set_status(&self, status: bool) {
        *self.status.write().unwrap() = status;
    }

    pub fn set_temperature(&self, temp: f32) {
        *self.temperature.write().unwrap() = temp;
    }

    pub fn set_high_threshold(&self, threshold: f32) {
        *self.high_threshold.write().unwrap() = threshold;
    }

    pub fn set_low_threshold(&self, threshold: f32) {
        *self.low_threshold.write().unwrap() = threshold;
    }

    pub fn make_over_temper(&self) {
        *self.high_threshold.write().unwrap() = 2.0;
        *self.temperature.write().unwrap() = 3.0;
        *self.low_threshold.write().unwrap() = 1.0;
    }

    pub fn make_under_temper(&self) {
        *self.high_threshold.write().unwrap() = 3.0;
        *self.temperature.write().unwrap() = 1.0;
        *self.low_threshold.write().unwrap() = 2.0;
    }

    pub fn make_normal_temper(&self) {
        *self.high_threshold.write().unwrap() = 3.0;
        *self.temperature.write().unwrap() = 2.0;
        *self.low_threshold.write().unwrap() = 1.0;
    }
}

impl Default for MockThermal {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Thermal for MockThermal {
    fn get_name(&self) -> Result<String> {
        Ok(self.name.read().unwrap().clone().unwrap_or_default())
    }

    fn get_temperature(&self) -> Result<f32> {
        Ok(*self.temperature.read().unwrap())
    }

    fn get_high_threshold(&self) -> Result<f32> {
        Ok(*self.high_threshold.read().unwrap())
    }

    fn get_low_threshold(&self) -> Result<f32> {
        Ok(*self.low_threshold.read().unwrap())
    }

    fn get_high_critical_threshold(&self) -> Result<f32> {
        Ok(*self.high_critical_threshold.read().unwrap())
    }

    fn get_low_critical_threshold(&self) -> Result<f32> {
        Ok(*self.low_critical_threshold.read().unwrap())
    }

    fn get_minimum_recorded(&self) -> Result<f32> {
        Ok(*self.minimum_temperature.read().unwrap())
    }

    fn get_maximum_recorded(&self) -> Result<f32> {
        Ok(*self.maximum_temperature.read().unwrap())
    }

    fn is_replaceable(&self) -> Result<bool> {
        Ok(*self.replaceable.read().unwrap())
    }

    fn get_position_in_parent(&self) -> Result<usize> {
        Ok(*self.position_in_parent.read().unwrap())
    }
}

pub struct MockErrorThermal {
    base: MockThermal,
}

impl MockErrorThermal {
    pub fn new() -> Self {
        Self {
            base: MockThermal::new(None),
        }
    }
}

impl Default for MockErrorThermal {
    fn default() -> Self {
        Self::new()
    }
}

impl Thermal for MockErrorThermal {
    fn get_name(&self) -> Result<String> {
        self.base.get_name()
    }

    fn get_temperature(&self) -> Result<f32> {
        Err(anyhow::anyhow!("Failed to get temperature"))
    }

    fn get_high_threshold(&self) -> Result<f32> {
        self.base.get_high_threshold()
    }

    fn get_low_threshold(&self) -> Result<f32> {
        self.base.get_low_threshold()
    }

    fn get_high_critical_threshold(&self) -> Result<f32> {
        self.base.get_high_critical_threshold()
    }

    fn get_low_critical_threshold(&self) -> Result<f32> {
        self.base.get_low_critical_threshold()
    }

    fn get_minimum_recorded(&self) -> Result<f32> {
        self.base.get_minimum_recorded()
    }

    fn get_maximum_recorded(&self) -> Result<f32> {
        self.base.get_maximum_recorded()
    }

    fn is_replaceable(&self) -> Result<bool> {
        self.base.is_replaceable()
    }

    fn get_position_in_parent(&self) -> Result<usize> {
        self.base.get_position_in_parent()
    }
}

pub struct MockThermalManager {}

impl MockThermalManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MockThermalManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MockModule {
    sfp_list: RwLock<Vec<MockSfp>>,
    psu_list: RwLock<Vec<MockPsu>>,
    fan_list: RwLock<Vec<Box<dyn Fan>>>,
    thermal_list: RwLock<Vec<Box<dyn Thermal>>>,
}

impl MockModule {
    pub fn new() -> Self {
        Self {
            sfp_list: RwLock::new(Vec::new()),
            psu_list: RwLock::new(Vec::new()),
            fan_list: RwLock::new(Vec::new()),
            thermal_list: RwLock::new(Vec::new()),
        }
    }

    pub fn add_sfp(&self, sfp: MockSfp) {
        self.sfp_list.write().unwrap().push(sfp);
    }

    pub fn add_psu(&self, psu: MockPsu) {
        self.psu_list.write().unwrap().push(psu);
    }

    pub fn add_fan(&self, fan: Box<dyn Fan>) {
        self.fan_list.write().unwrap().push(fan);
    }

    pub fn add_thermal(&self, thermal: Box<dyn Thermal>) {
        self.thermal_list.write().unwrap().push(thermal);
    }

    pub fn get_all_sfps(&self) -> std::sync::RwLockReadGuard<Vec<MockSfp>> {
        self.sfp_list.read().unwrap()
    }

    pub fn get_all_psus(&self) -> std::sync::RwLockReadGuard<Vec<MockPsu>> {
        self.psu_list.read().unwrap()
    }

    pub fn get_all_fans(&self) -> std::sync::RwLockReadGuard<Vec<Box<dyn Fan>>> {
        self.fan_list.read().unwrap()
    }

    pub fn get_all_thermals(&self) -> std::sync::RwLockReadGuard<Vec<Box<dyn Thermal>>> {
        self.thermal_list.read().unwrap()
    }
}

impl Default for MockModule {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MockChassis {
    name: RwLock<Option<String>>,
    presence: RwLock<bool>,
    model: String,
    serial: String,
    status: RwLock<bool>,
    position_in_parent: usize,
    replaceable: bool,
    is_chassis_system: RwLock<bool>,
    is_dpu: RwLock<bool>,
    is_smartswitch: RwLock<bool>,
    my_slot: RwLock<i32>,
    dpu_id: RwLock<Option<i32>>,
    thermal_manager: MockThermalManager,
    fan_list: RwLock<Vec<Box<dyn Fan>>>,
    fan_drawer_list: RwLock<Vec<MockFanDrawer>>,
    thermal_list: RwLock<Vec<Box<dyn Thermal>>>,
    psu_list: RwLock<Vec<MockPsu>>,
    module_list: RwLock<Vec<MockModule>>,
}

impl MockChassis {
    pub fn new() -> Self {
        Self {
            name: RwLock::new(None),
            presence: RwLock::new(true),
            model: "Chassis Model".to_string(),
            serial: "Chassis Serial".to_string(),
            status: RwLock::new(true),
            position_in_parent: 1,
            replaceable: false,
            is_chassis_system: RwLock::new(false),
            is_dpu: RwLock::new(false),
            is_smartswitch: RwLock::new(false),
            my_slot: RwLock::new(MODULE_INVALID_SLOT),
            dpu_id: RwLock::new(None),
            thermal_manager: MockThermalManager::new(),
            fan_list: RwLock::new(Vec::new()),
            fan_drawer_list: RwLock::new(Vec::new()),
            thermal_list: RwLock::new(Vec::new()),
            psu_list: RwLock::new(Vec::new()),
            module_list: RwLock::new(Vec::new()),
        }
    }

    pub fn make_absent_fan(&self) {
        let fan = MockFan::new();
        fan.set_presence(false);
        let drawer_index = self.fan_drawer_list.read().unwrap().len();
        let fan_drawer = MockFanDrawer::new(drawer_index);
        fan_drawer.add_fan(Box::new(MockFan::new()));
        fan_drawer.get_all_fans().last().map(|_| {
            let mut fans = self.fan_list.write().unwrap();
            fans.push(Box::new({
                let f = MockFan::new();
                f.set_presence(false);
                f
            }));
        });
        self.fan_drawer_list.write().unwrap().push(fan_drawer);
    }

    pub fn make_faulty_fan(&self) {
        let fan = MockFan::new();
        fan.set_status(false);
        let drawer_index = self.fan_drawer_list.read().unwrap().len();
        let fan_drawer = MockFanDrawer::new(drawer_index);
        self.fan_list.write().unwrap().push(Box::new({
            let f = MockFan::new();
            f.set_status(false);
            f
        }));
        self.fan_drawer_list.write().unwrap().push(fan_drawer);
    }

    pub fn make_under_speed_fan(&self) {
        let fan = MockFan::new();
        fan.make_under_speed();
        let drawer_index = self.fan_drawer_list.read().unwrap().len();
        let fan_drawer = MockFanDrawer::new(drawer_index);
        self.fan_list.write().unwrap().push(Box::new({
            let f = MockFan::new();
            f.make_under_speed();
            f
        }));
        self.fan_drawer_list.write().unwrap().push(fan_drawer);
    }

    pub fn make_over_speed_fan(&self) {
        let fan = MockFan::new();
        fan.make_over_speed();
        let drawer_index = self.fan_drawer_list.read().unwrap().len();
        let fan_drawer = MockFanDrawer::new(drawer_index);
        self.fan_list.write().unwrap().push(Box::new({
            let f = MockFan::new();
            f.make_over_speed();
            f
        }));
        self.fan_drawer_list.write().unwrap().push(fan_drawer);
    }

    pub fn make_error_fan(&self) {
        let drawer_index = self.fan_drawer_list.read().unwrap().len();
        let fan_drawer = MockFanDrawer::new(drawer_index);
        self.fan_list.write().unwrap().push(Box::new(MockErrorFan::new()));
        self.fan_drawer_list.write().unwrap().push(fan_drawer);
    }

    pub fn make_over_temper_thermal(&self) {
        let thermal = MockThermal::new(None);
        thermal.make_over_temper();
        self.thermal_list.write().unwrap().push(Box::new(thermal));
    }

    pub fn make_under_temper_thermal(&self) {
        let thermal = MockThermal::new(None);
        thermal.make_under_temper();
        self.thermal_list.write().unwrap().push(Box::new(thermal));
    }

    pub fn make_error_thermal(&self) {
        self.thermal_list.write().unwrap().push(Box::new(MockErrorThermal::new()));
    }

    pub fn make_module_thermal(&self) {
        let module = MockModule::new();

        let sfp = MockSfp::new();
        sfp.add_thermal(Box::new(MockThermal::new(None)));
        module.add_sfp(sfp);

        let psu = MockPsu::new();
        psu.add_thermal(Box::new(MockThermal::new(None)));
        module.add_psu(psu);

        module.add_fan(Box::new(MockFan::new()));
        module.add_thermal(Box::new(MockThermal::new(None)));

        self.module_list.write().unwrap().push(module);
    }

    pub fn is_modular_chassis(&self) -> bool {
        *self.is_chassis_system.read().unwrap()
    }

    pub fn set_modular_chassis(&self, is_true: bool) {
        *self.is_chassis_system.write().unwrap() = is_true;
    }

    pub fn set_my_slot(&self, my_slot: i32) {
        *self.my_slot.write().unwrap() = my_slot;
    }

    pub fn get_my_slot(&self) -> i32 {
        *self.my_slot.read().unwrap()
    }

    pub fn get_thermal_manager(&self) -> &MockThermalManager {
        &self.thermal_manager
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.read().unwrap().clone()
    }

    pub fn get_presence(&self) -> bool {
        *self.presence.read().unwrap()
    }

    pub fn set_presence(&self, presence: bool) {
        *self.presence.write().unwrap() = presence;
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }

    pub fn get_serial(&self) -> &str {
        &self.serial
    }

    pub fn get_status(&self) -> bool {
        *self.status.read().unwrap()
    }

    pub fn set_status(&self, status: bool) {
        *self.status.write().unwrap() = status;
    }

    pub fn get_position_in_parent(&self) -> usize {
        self.position_in_parent
    }

    pub fn is_replaceable(&self) -> bool {
        self.replaceable
    }

    pub fn is_dpu(&self) -> bool {
        *self.is_dpu.read().unwrap()
    }

    pub fn is_smartswitch(&self) -> bool {
        *self.is_smartswitch.read().unwrap()
    }

    pub fn set_smartswitch(&self, is_true: bool) {
        *self.is_smartswitch.write().unwrap() = is_true;
    }

    pub fn set_dpu(&self, is_true: bool) {
        *self.is_dpu.write().unwrap() = is_true;
    }

    pub fn set_dpu_id(&self, dpu_id: i32) {
        *self.dpu_id.write().unwrap() = Some(dpu_id);
    }

    pub fn get_dpu_id(&self) -> Result<i32> {
        self.dpu_id.read().unwrap().ok_or_else(|| anyhow::anyhow!("Not implemented"))
    }

    pub fn get_all_fans(&self) -> std::sync::RwLockReadGuard<Vec<Box<dyn Fan>>> {
        self.fan_list.read().unwrap()
    }

    pub fn get_all_fan_drawers(&self) -> std::sync::RwLockReadGuard<Vec<MockFanDrawer>> {
        self.fan_drawer_list.read().unwrap()
    }

    pub fn get_all_thermals(&self) -> std::sync::RwLockReadGuard<Vec<Box<dyn Thermal>>> {
        self.thermal_list.read().unwrap()
    }

    pub fn get_all_psus(&self) -> std::sync::RwLockReadGuard<Vec<MockPsu>> {
        self.psu_list.read().unwrap()
    }

    pub fn get_all_modules(&self) -> std::sync::RwLockReadGuard<Vec<MockModule>> {
        self.module_list.read().unwrap()
    }

    pub fn add_fan(&self, fan: Box<dyn Fan>) {
        self.fan_list.write().unwrap().push(fan);
    }

    pub fn add_fan_drawer(&self, drawer: MockFanDrawer) {
        self.fan_drawer_list.write().unwrap().push(drawer);
    }

    pub fn add_thermal(&self, thermal: Box<dyn Thermal>) {
        self.thermal_list.write().unwrap().push(thermal);
    }

    pub fn add_psu(&self, psu: MockPsu) {
        self.psu_list.write().unwrap().push(psu);
    }

    pub fn add_module(&self, module: MockModule) {
        self.module_list.write().unwrap().push(module);
    }
}

impl Default for MockChassis {
    fn default() -> Self {
        Self::new()
    }
}