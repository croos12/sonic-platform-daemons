mod mock_platform;

use mock_platform::*;
use sonic_platform::{Fan, FanStatus, LedColor, Thermal, TemperatureStatus};

const NOT_AVAILABLE: f32 = -999.0;

#[test]
fn test_set_presence() {
    FanStatus::reset_fan_counter();
    let mut fan_status = FanStatus::new();

    let ret = fan_status.set_presence(true);
    assert!(!ret);
    assert!(fan_status.presence);

    let ret = fan_status.set_presence(false);
    assert!(ret);
    assert!(!fan_status.presence);
}

#[test]
fn test_set_under_speed() {
    let mut fan_status = FanStatus::new();

    let ret = fan_status.set_under_speed(false);
    assert!(!ret);

    let ret = fan_status.set_under_speed(true);
    assert!(ret);
    assert!(fan_status.under_speed);
    assert!(!fan_status.is_ok());

    let ret = fan_status.set_under_speed(true);
    assert!(!ret);

    let ret = fan_status.set_under_speed(false);
    assert!(ret);
    assert!(!fan_status.under_speed);
    assert!(fan_status.is_ok());

    let ret = fan_status.set_under_speed(false);
    assert!(!ret);
}

#[test]
fn test_set_over_speed() {
    let mut fan_status = FanStatus::new();

    let ret = fan_status.set_over_speed(false);
    assert!(!ret);

    let ret = fan_status.set_over_speed(true);
    assert!(ret);
    assert!(fan_status.over_speed);
    assert!(!fan_status.is_ok());

    let ret = fan_status.set_over_speed(true);
    assert!(!ret);

    let ret = fan_status.set_over_speed(false);
    assert!(ret);
    assert!(!fan_status.over_speed);
    assert!(fan_status.is_ok());

    let ret = fan_status.set_over_speed(false);
    assert!(!ret);
}

#[test]
fn test_insufficient_fan_number() {
    FanStatus::reset_fan_counter();

    let mut fan_status1 = FanStatus::new();
    let mut fan_status2 = FanStatus::new();

    fan_status1.set_presence(false);
    fan_status2.set_fault_status(false);

    assert_eq!(FanStatus::get_bad_fan_count(), 2);

    FanStatus::reset_fan_counter();
    assert_eq!(FanStatus::get_bad_fan_count(), 0);
}

#[test]
fn test_mock_fan_presence() {
    let fan = MockFan::new();
    assert!(fan.get_presence().unwrap());

    fan.set_presence(false);
    assert!(!fan.get_presence().unwrap());

    fan.set_presence(true);
    assert!(fan.get_presence().unwrap());
}

#[test]
fn test_mock_fan_under_speed() {
    let fan = MockFan::new();
    assert!(!fan.is_under_speed().unwrap());

    fan.make_under_speed();
    assert!(fan.is_under_speed().unwrap());
    assert_eq!(fan.get_speed().unwrap(), 1);
    assert_eq!(fan.get_target_speed().unwrap(), 2);

    fan.make_normal_speed();
    assert!(!fan.is_under_speed().unwrap());
}

#[test]
fn test_mock_fan_over_speed() {
    let fan = MockFan::new();
    assert!(!fan.is_over_speed().unwrap());

    fan.make_over_speed();
    assert!(fan.is_over_speed().unwrap());
    assert_eq!(fan.get_speed().unwrap(), 2);
    assert_eq!(fan.get_target_speed().unwrap(), 1);

    fan.make_normal_speed();
    assert!(!fan.is_over_speed().unwrap());
}

#[test]
fn test_mock_error_fan() {
    let fan = MockErrorFan::new();

    assert!(fan.get_presence().is_ok());
    assert!(fan.get_status().is_ok());

    assert!(fan.get_speed().is_err());
    let err = fan.get_speed().unwrap_err();
    assert_eq!(err.to_string(), "Failed to get speed");
}

#[test]
fn test_mock_fan_led() {
    let fan = MockFan::new();
    assert_eq!(fan.get_status_led().unwrap(), LedColor::Red);

    fan.set_status_led(LedColor::Green).unwrap();
    assert_eq!(fan.get_status_led().unwrap(), LedColor::Green);

    fan.set_status_led(LedColor::Amber).unwrap();
    assert_eq!(fan.get_status_led().unwrap(), LedColor::Amber);

    fan.set_status_led(LedColor::Off).unwrap();
    assert_eq!(fan.get_status_led().unwrap(), LedColor::Off);
}

#[test]
fn test_temperature_status_set_over_temper() {
    let mut temperature_status = TemperatureStatus::new();

    let ret = temperature_status.set_over_temperature(NOT_AVAILABLE, NOT_AVAILABLE);
    assert!(!ret);

    let ret = temperature_status.set_over_temperature(NOT_AVAILABLE, 0.0);
    assert!(!ret);

    let ret = temperature_status.set_over_temperature(0.0, NOT_AVAILABLE);
    assert!(!ret);

    let ret = temperature_status.set_over_temperature(2.0, 1.0);
    assert!(ret);
    assert!(temperature_status.over_temperature);

    let ret = temperature_status.set_over_temperature(1.0, 2.0);
    assert!(ret);
    assert!(!temperature_status.over_temperature);
}

#[test]
fn test_temperature_status_set_under_temper() {
    let mut temperature_status = TemperatureStatus::new();

    let ret = temperature_status.set_under_temperature(NOT_AVAILABLE, NOT_AVAILABLE);
    assert!(!ret);

    let ret = temperature_status.set_under_temperature(NOT_AVAILABLE, 0.0);
    assert!(!ret);

    let ret = temperature_status.set_under_temperature(0.0, NOT_AVAILABLE);
    assert!(!ret);

    let ret = temperature_status.set_under_temperature(1.0, 2.0);
    assert!(ret);
    assert!(temperature_status.under_temperature);

    let ret = temperature_status.set_under_temperature(2.0, 1.0);
    assert!(ret);
    assert!(!temperature_status.under_temperature);
}

#[test]
fn test_temperature_status_set_not_available() {
    let thermal_name = "Chassis 1 Thermal 1";
    let mut temperature_status = TemperatureStatus::new();
    temperature_status.temperature = Some(20.0);

    temperature_status.set_temperature(thermal_name, NOT_AVAILABLE);
    assert!(temperature_status.temperature.is_some());
    assert_eq!(temperature_status.temperature.unwrap(), NOT_AVAILABLE);
}

#[test]
fn test_temperature_status_set_temperature() {
    let thermal_name = "Chassis 1 Thermal 1";
    let mut temperature_status = TemperatureStatus::new();

    let changed = temperature_status.set_temperature(thermal_name, 20.0);
    assert!(changed);
    assert_eq!(temperature_status.temperature, Some(20.0));

    let changed = temperature_status.set_temperature(thermal_name, 20.05);
    assert!(!changed);

    let changed = temperature_status.set_temperature(thermal_name, 25.0);
    assert!(changed);
    assert_eq!(temperature_status.temperature, Some(25.0));
}

#[test]
fn test_mock_thermal_over_temper() {
    let thermal = MockThermal::new(Some(1));

    thermal.make_over_temper();
    let temp = thermal.get_temperature().unwrap();
    let high_threshold = thermal.get_high_threshold().unwrap();
    assert!(temp > high_threshold);
    assert_eq!(temp, 3.0);
    assert_eq!(high_threshold, 2.0);
}

#[test]
fn test_mock_thermal_under_temper() {
    let thermal = MockThermal::new(Some(1));

    thermal.make_under_temper();
    let temp = thermal.get_temperature().unwrap();
    let low_threshold = thermal.get_low_threshold().unwrap();
    assert!(temp < low_threshold);
    assert_eq!(temp, 1.0);
    assert_eq!(low_threshold, 2.0);
}

#[test]
fn test_mock_thermal_normal_temper() {
    let thermal = MockThermal::new(Some(1));

    thermal.make_normal_temper();
    let temp = thermal.get_temperature().unwrap();
    let high_threshold = thermal.get_high_threshold().unwrap();
    let low_threshold = thermal.get_low_threshold().unwrap();

    assert!(temp >= low_threshold);
    assert!(temp <= high_threshold);
    assert_eq!(temp, 2.0);
    assert_eq!(high_threshold, 3.0);
    assert_eq!(low_threshold, 1.0);
}

#[test]
fn test_mock_error_thermal() {
    let thermal = MockErrorThermal::new();

    assert!(thermal.get_name().is_ok());
    assert!(thermal.get_high_threshold().is_ok());

    assert!(thermal.get_temperature().is_err());
    let err = thermal.get_temperature().unwrap_err();
    assert_eq!(err.to_string(), "Failed to get temperature");
}

#[test]
fn test_mock_thermal_thresholds() {
    let thermal = MockThermal::new(Some(1));

    assert_eq!(thermal.get_temperature().unwrap(), 2.0);
    assert_eq!(thermal.get_minimum_recorded().unwrap(), 1.0);
    assert_eq!(thermal.get_maximum_recorded().unwrap(), 5.0);
    assert_eq!(thermal.get_high_threshold().unwrap(), 3.0);
    assert_eq!(thermal.get_low_threshold().unwrap(), 1.0);
    assert_eq!(thermal.get_high_critical_threshold().unwrap(), 4.0);
    assert_eq!(thermal.get_low_critical_threshold().unwrap(), 0.0);
}

#[test]
fn test_mock_chassis_fans() {
    let chassis = MockChassis::new();

    assert_eq!(chassis.get_all_fans().len(), 0);
    assert_eq!(chassis.get_all_fan_drawers().len(), 0);

    chassis.make_absent_fan();
    assert_eq!(chassis.get_all_fan_drawers().len(), 1);

    chassis.make_faulty_fan();
    assert_eq!(chassis.get_all_fan_drawers().len(), 2);

    chassis.make_under_speed_fan();
    assert_eq!(chassis.get_all_fan_drawers().len(), 3);

    chassis.make_over_speed_fan();
    assert_eq!(chassis.get_all_fan_drawers().len(), 4);

    chassis.make_error_fan();
    assert_eq!(chassis.get_all_fan_drawers().len(), 5);
}

#[test]
fn test_mock_chassis_thermals() {
    let chassis = MockChassis::new();

    assert_eq!(chassis.get_all_thermals().len(), 0);

    chassis.make_over_temper_thermal();
    assert_eq!(chassis.get_all_thermals().len(), 1);

    chassis.make_under_temper_thermal();
    assert_eq!(chassis.get_all_thermals().len(), 2);

    chassis.make_error_thermal();
    assert_eq!(chassis.get_all_thermals().len(), 3);
}

#[test]
fn test_mock_chassis_modular() {
    let chassis = MockChassis::new();

    assert!(!chassis.is_modular_chassis());

    chassis.set_modular_chassis(true);
    assert!(chassis.is_modular_chassis());

    chassis.set_my_slot(1);
    assert_eq!(chassis.get_my_slot(), 1);

    chassis.set_my_slot(-1);
    assert_eq!(chassis.get_my_slot(), -1);
}

#[test]
fn test_mock_chassis_dpu() {
    let chassis = MockChassis::new();

    assert!(!chassis.is_dpu());
    assert!(!chassis.is_smartswitch());

    chassis.set_dpu(true);
    assert!(chassis.is_dpu());

    chassis.set_smartswitch(true);
    assert!(chassis.is_smartswitch());

    assert!(chassis.get_dpu_id().is_err());

    chassis.set_dpu_id(1);
    assert_eq!(chassis.get_dpu_id().unwrap(), 1);
}

#[test]
fn test_mock_chassis_module_thermal() {
    let chassis = MockChassis::new();
    chassis.make_module_thermal();

    let modules = chassis.get_all_modules();
    assert_eq!(modules.len(), 1);

    let module = &modules[0];
    assert_eq!(module.get_all_sfps().len(), 1);
    assert_eq!(module.get_all_psus().len(), 1);
    assert_eq!(module.get_all_fans().len(), 1);
    assert_eq!(module.get_all_thermals().len(), 1);

    let sfp = &module.get_all_sfps()[0];
    assert_eq!(sfp.get_all_thermals().len(), 1);

    let psu = &module.get_all_psus()[0];
    assert_eq!(psu.get_all_thermals().len(), 1);
}

#[test]
fn test_mock_fan_drawer() {
    let drawer = MockFanDrawer::new(0);

    assert_eq!(drawer.get_name(), "FanDrawer 0");
    assert!(drawer.get_presence());
    assert!(drawer.get_status());
    assert_eq!(drawer.get_model(), "Fan Drawer Model");
    assert_eq!(drawer.get_serial(), "Fan Drawer Serial");
    assert!(drawer.is_replaceable());
    assert_eq!(drawer.get_position_in_parent(), 1);

    assert_eq!(drawer.get_all_fans().len(), 0);

    drawer.add_fan(Box::new(MockFan::new()));
    assert_eq!(drawer.get_all_fans().len(), 1);
}

#[test]
fn test_mock_fan_drawer_led() {
    let drawer = MockFanDrawer::new(0);

    assert_eq!(drawer.get_status_led(), LedColor::Red);

    drawer.set_status_led(LedColor::Green);
    assert_eq!(drawer.get_status_led(), LedColor::Green);
}

#[test]
fn test_mock_psu() {
    let psu = MockPsu::new();

    assert!(psu.get_presence());
    assert!(psu.get_status());
    assert!(psu.get_powergood_status());
    assert_eq!(psu.get_model(), "PSU Model");
    assert_eq!(psu.get_serial(), "PSU Serial");
    assert!(psu.is_replaceable());
    assert_eq!(psu.get_position_in_parent(), 1);

    assert_eq!(psu.get_all_fans().len(), 0);
    assert_eq!(psu.get_all_thermals().len(), 0);

    psu.add_fan(Box::new(MockFan::new()));
    psu.add_thermal(Box::new(MockThermal::new(Some(1))));

    assert_eq!(psu.get_all_fans().len(), 1);
    assert_eq!(psu.get_all_thermals().len(), 1);
}

#[test]
fn test_mock_sfp() {
    let sfp = MockSfp::new();

    assert!(sfp.get_presence());
    assert!(sfp.get_status());
    assert_eq!(sfp.get_model(), "SFP Model");
    assert_eq!(sfp.get_serial(), "SFP Serial");
    assert!(sfp.is_replaceable());
    assert_eq!(sfp.get_position_in_parent(), 1);

    assert_eq!(sfp.get_all_thermals().len(), 0);

    sfp.add_thermal(Box::new(MockThermal::new(Some(1))));
    assert_eq!(sfp.get_all_thermals().len(), 1);
}

#[test]
fn test_mock_module() {
    let module = MockModule::new();

    assert_eq!(module.get_all_sfps().len(), 0);
    assert_eq!(module.get_all_psus().len(), 0);
    assert_eq!(module.get_all_fans().len(), 0);
    assert_eq!(module.get_all_thermals().len(), 0);

    module.add_sfp(MockSfp::new());
    module.add_psu(MockPsu::new());
    module.add_fan(Box::new(MockFan::new()));
    module.add_thermal(Box::new(MockThermal::new(Some(1))));

    assert_eq!(module.get_all_sfps().len(), 1);
    assert_eq!(module.get_all_psus().len(), 1);
    assert_eq!(module.get_all_fans().len(), 1);
    assert_eq!(module.get_all_thermals().len(), 1);
}
