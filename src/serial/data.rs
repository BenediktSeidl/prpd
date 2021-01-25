use std::collections::HashMap;

#[derive(Clone)]
pub struct SerialParameterSpec {
    pub name: String,
    pub factor: f64,
    pub class: String,
    pub unit_of_measurement: String,
}

pub type SerialSpecs = HashMap<u16, SerialParameterSpec>;

pub fn get_specs() -> SerialSpecs {
    return [
        (
            0x0,
            SerialParameterSpec {
                class: "voltage".into(),
                name: "l1".into(),
                unit_of_measurement: "V".into(),
                factor: 0.1,
            },
        ),
        (
            0x2,
            SerialParameterSpec {
                class: "voltage".into(),
                name: "l2".into(),
                unit_of_measurement: "V".into(),
                factor: 0.1,
            },
        ),
        (
            0x4,
            SerialParameterSpec {
                class: "voltage".into(),
                name: "l3".into(),
                unit_of_measurement: "V".into(),
                factor: 0.1,
            },
        ),
        (
            0xc,
            SerialParameterSpec {
                class: "current".into(),
                name: "l1".into(),
                unit_of_measurement: "A".into(),
                factor: 0.001,
            },
        ),
        (
            0xe,
            SerialParameterSpec {
                class: "current".into(),
                name: "l2".into(),
                unit_of_measurement: "A".into(),
                factor: 0.001,
            },
        ),
        (
            0x10,
            SerialParameterSpec {
                class: "current".into(),
                name: "l3".into(),
                unit_of_measurement: "A".into(),
                factor: 0.001,
            },
        ),
        (
            0x12,
            SerialParameterSpec {
                class: "power".into(),
                name: "l1".into(),
                unit_of_measurement: "W".into(),
                factor: 0.1,
            },
        ),
        (
            0x14,
            SerialParameterSpec {
                class: "power".into(),
                name: "l2".into(),
                unit_of_measurement: "W".into(),
                factor: 0.1,
            },
        ),
        (
            0x46,
            SerialParameterSpec {
                class: "energy".into(),
                name: "l1".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.1,
            },
        ),
        (
            0x48,
            SerialParameterSpec {
                class: "energy".into(),
                name: "l2".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.1,
            },
        ),
        (
            0x4a,
            SerialParameterSpec {
                class: "energy".into(),
                name: "l3".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.1,
            },
        ),
        (
            0x16,
            SerialParameterSpec {
                class: "power".into(),
                name: "l3".into(),
                unit_of_measurement: "W".into(),
                factor: 0.1,
            },
        ),
        (
            0x18,
            SerialParameterSpec {
                class: "power".into(),
                name: "l1".into(),
                unit_of_measurement: "VA".into(),
                factor: 0.1,
            },
        ),
        (
            0x1a,
            SerialParameterSpec {
                class: "power".into(),
                name: "l2".into(),
                unit_of_measurement: "VA".into(),
                factor: 0.1,
            },
        ),
        (
            0x1c,
            SerialParameterSpec {
                class: "power".into(),
                name: "l3".into(),
                unit_of_measurement: "VA".into(),
                factor: 0.1,
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();
}
