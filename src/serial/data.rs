use std::collections::HashMap;

use crate::output::{Phase, UnitOfMeasurement};

#[derive(Clone)]
pub struct SerialParameterSpec {
    pub name: String,
    pub factor: f64,
    pub unit_of_measurement: UnitOfMeasurement,
    pub phase: Phase,
}

pub type SerialSpecs = HashMap<u16, SerialParameterSpec>;

pub fn get_specs() -> SerialSpecs {
    return [
        (
            0x0,
            SerialParameterSpec {
                name: "l1".into(),
                phase: Phase::L1,
                unit_of_measurement: UnitOfMeasurement::V,
                factor: 0.1,
            },
        ),
        (
            0x2,
            SerialParameterSpec {
                name: "l2".into(),
                phase: Phase::L2,
                unit_of_measurement: UnitOfMeasurement::V,
                factor: 0.1,
            },
        ),
        (
            0x4,
            SerialParameterSpec {
                name: "l3".into(),
                phase: Phase::L3,
                unit_of_measurement: UnitOfMeasurement::V,
                factor: 0.1,
            },
        ),
        (
            0xc,
            SerialParameterSpec {
                name: "l1".into(),
                phase: Phase::L1,
                unit_of_measurement: UnitOfMeasurement::A,
                factor: 0.001,
            },
        ),
        (
            0xe,
            SerialParameterSpec {
                name: "l2".into(),
                phase: Phase::L2,
                unit_of_measurement: UnitOfMeasurement::A,
                factor: 0.001,
            },
        ),
        (
            0x10,
            SerialParameterSpec {
                name: "l3".into(),
                phase: Phase::L3,
                unit_of_measurement: UnitOfMeasurement::A,
                factor: 0.001,
            },
        ),
        (
            0x12,
            SerialParameterSpec {
                name: "l1".into(),
                phase: Phase::L1,
                unit_of_measurement: UnitOfMeasurement::W,
                factor: 0.1,
            },
        ),
        (
            0x14,
            SerialParameterSpec {
                name: "l2".into(),
                phase: Phase::L2,
                unit_of_measurement: UnitOfMeasurement::W,
                factor: 0.1,
            },
        ),
        (
            0x46,
            SerialParameterSpec {
                name: "l1".into(),
                phase: Phase::L1,
                unit_of_measurement: UnitOfMeasurement::Wh,
                factor: 0.1 * 1000.0,
            },
        ),
        (
            0x48,
            SerialParameterSpec {
                name: "l2".into(),
                phase: Phase::L2,
                unit_of_measurement: UnitOfMeasurement::Wh,
                factor: 0.1 * 1000.0,
            },
        ),
        (
            0x4a,
            SerialParameterSpec {
                name: "l3".into(),
                phase: Phase::L3,
                unit_of_measurement: UnitOfMeasurement::Wh,
                factor: 0.1 * 1000.0,
            },
        ),
        (
            0x16,
            SerialParameterSpec {
                name: "l3".into(),
                phase: Phase::L3,
                unit_of_measurement: UnitOfMeasurement::W,
                factor: 0.1,
            },
        ),
        (
            0x18,
            SerialParameterSpec {
                name: "l1".into(),
                phase: Phase::L1,
                unit_of_measurement: UnitOfMeasurement::VA,
                factor: 0.1,
            },
        ),
        (
            0x1a,
            SerialParameterSpec {
                name: "l2".into(),
                phase: Phase::L2,
                unit_of_measurement: UnitOfMeasurement::VA,
                factor: 0.1,
            },
        ),
        (
            0x1c,
            SerialParameterSpec {
                name: "l3".into(),
                phase: Phase::L3,
                unit_of_measurement: UnitOfMeasurement::VA,
                factor: 0.1,
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();
}
