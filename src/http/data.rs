// the information of KNOWN_PARAMETERS is extracted from:
// https://github.com/trebb/p-rout/blob/master/p-rout-view.scm
// and therefore the following license applies:

// Copyright (c) 2014, 2015 Bert Burgemeister  trebbu@googlemail.com
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use,
// copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE./

use std::collections::HashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct HttpIndex {
    pub module_id: i64,
    pub param_id: i64,
}

#[derive(Clone)]
pub struct HttpParameterSpec {
    pub name: String,
    pub factor: f64,
    pub class: String,
    pub unit_of_measurement: String,
}

pub type HttpSpecs = HashMap<HttpIndex, HttpParameterSpec>;

pub fn get_specs() -> HttpSpecs {
    return [
        (
            HttpIndex {
                module_id: 9,
                param_id: 0,
            },
            HttpParameterSpec {
                name: "f_dcac".into(),
                unit_of_measurement: "Hz".into(),
                factor: 0.01,
                class: "None".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 9,
                param_id: 1,
            },
            HttpParameterSpec {
                name: "V_grid_dcac".into(),
                unit_of_measurement: "V".into(),
                factor: 0.1,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 9,
                param_id: 2,
            },
            HttpParameterSpec {
                name: "P_grid_dcac".into(),
                unit_of_measurement: "W".into(),
                factor: 1.0,
                class: "power".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 9,
                param_id: 3,
            },
            HttpParameterSpec {
                name: "W_dcac_produced".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 9,
                param_id: 4,
            },
            HttpParameterSpec {
                name: "W_dcac_consumed".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 9,
                param_id: 5,
            },
            HttpParameterSpec {
                name: "V_local".into(),
                unit_of_measurement: "V".into(),
                factor: 0.1,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 9,
                param_id: 6,
            },
            HttpParameterSpec {
                name: "P_local".into(),
                unit_of_measurement: "W".into(),
                factor: 1.0,
                class: "power".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 9,
                param_id: 7,
            },
            HttpParameterSpec {
                name: "W_local_dcac_consumed".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 9,
                param_id: 8,
            },
            HttpParameterSpec {
                name: "V_bus_dcac".into(),
                unit_of_measurement: "V".into(),
                factor: 0.01,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 9,
                param_id: 10,
            },
            HttpParameterSpec {
                name: "T_dcac".into(),
                unit_of_measurement: "°C".into(),
                factor: 0.1,
                class: "temperature".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 0,
            },
            HttpParameterSpec {
                name: "V_L1".into(),
                unit_of_measurement: "V".into(),
                factor: 0.1,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 1,
            },
            HttpParameterSpec {
                name: "I_L1".into(),
                unit_of_measurement: "A".into(),
                factor: 0.01,
                class: "current".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 2,
            },
            HttpParameterSpec {
                name: "P_L1".into(),
                unit_of_measurement: "W".into(),
                factor: 1.0,
                class: "power".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 3,
            },
            HttpParameterSpec {
                name: "W_L1_consumed".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 4,
            },
            HttpParameterSpec {
                name: "V_L2".into(),
                unit_of_measurement: "V".into(),
                factor: 0.1,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 5,
            },
            HttpParameterSpec {
                name: "I_L2".into(),
                unit_of_measurement: "A".into(),
                factor: 0.01,
                class: "current".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 6,
            },
            HttpParameterSpec {
                name: "P_L2".into(),
                unit_of_measurement: "W".into(),
                factor: 1.0,
                class: "power".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 7,
            },
            HttpParameterSpec {
                name: "W_L2_consumed".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 8,
            },
            HttpParameterSpec {
                name: "V_L3".into(),
                unit_of_measurement: "V".into(),
                factor: 0.1,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 9,
            },
            HttpParameterSpec {
                name: "I_L3".into(),
                unit_of_measurement: "A".into(),
                factor: 0.01,
                class: "current".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 10,
            },
            HttpParameterSpec {
                name: "P_L3".into(),
                unit_of_measurement: "W".into(),
                factor: 1.0,
                class: "power".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 11,
                param_id: 11,
            },
            HttpParameterSpec {
                name: "W_L3_consumed".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 0,
            },
            HttpParameterSpec {
                name: "V_1_solar".into(),
                unit_of_measurement: "V".into(),
                factor: 0.01,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 1,
            },
            HttpParameterSpec {
                name: "I_1_solar".into(),
                unit_of_measurement: "A".into(),
                factor: 0.01,
                class: "current".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 2,
            },
            HttpParameterSpec {
                name: "P_1_solar".into(),
                unit_of_measurement: "W".into(),
                factor: 1.0,
                class: "power".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 3,
            },
            HttpParameterSpec {
                name: "W_solar_1_produced".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 4,
            },
            HttpParameterSpec {
                name: "T_1_solar".into(),
                unit_of_measurement: "°C".into(),
                factor: 0.1,
                class: "temperature".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 5,
            },
            HttpParameterSpec {
                name: "V_2_solar".into(),
                unit_of_measurement: "V".into(),
                factor: 0.01,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 6,
            },
            HttpParameterSpec {
                name: "I_2_solar".into(),
                unit_of_measurement: "A".into(),
                factor: 0.01,
                class: "current".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 7,
            },
            HttpParameterSpec {
                name: "P_2_solar".into(),
                unit_of_measurement: "W".into(),
                factor: 1.0,
                class: "power".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 8,
            },
            HttpParameterSpec {
                name: "W_solar_2_produced".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 9,
            },
            HttpParameterSpec {
                name: "T_2_solar".into(),
                unit_of_measurement: "°C".into(),
                factor: 0.1,
                class: "temperature".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 10,
            },
            HttpParameterSpec {
                name: "P_solar".into(),
                unit_of_measurement: "W".into(),
                factor: 1.0,
                class: "power".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 12,
                param_id: 11,
            },
            HttpParameterSpec {
                name: "W_solar_produced".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 16,
                param_id: 0,
            },
            HttpParameterSpec {
                name: "f_platform".into(),
                unit_of_measurement: "Hz".into(),
                factor: 0.01,
                class: "None".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 16,
                param_id: 1,
            },
            HttpParameterSpec {
                name: "V_grid_platform".into(),
                unit_of_measurement: "V".into(),
                factor: 0.1,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 16,
                param_id: 2,
            },
            HttpParameterSpec {
                name: "T_platform".into(),
                unit_of_measurement: "°C".into(),
                factor: 0.1,
                class: "temperature".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 16,
                param_id: 3,
            },
            HttpParameterSpec {
                name: "P_grid_platform".into(),
                unit_of_measurement: "W".into(),
                factor: 1.0,
                class: "power".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 16,
                param_id: 4,
            },
            HttpParameterSpec {
                name: "W_platform_produced".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 16,
                param_id: 5,
            },
            HttpParameterSpec {
                name: "W_platform_consumed".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 0,
            },
            HttpParameterSpec {
                name: "V_batt".into(),
                unit_of_measurement: "V".into(),
                factor: 0.01,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 1,
            },
            HttpParameterSpec {
                name: "I_batt".into(),
                unit_of_measurement: "A".into(),
                factor: 0.01,
                class: "current".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 2,
            },
            HttpParameterSpec {
                name: "P_batt".into(),
                unit_of_measurement: "W".into(),
                factor: 1.0,
                class: "power".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 2,
            },
            HttpParameterSpec {
                name: "P_bus_batt".into(),
                unit_of_measurement: "W".into(),
                factor: 1.0,
                class: "power".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 3,
            },
            HttpParameterSpec {
                name: "W_battery_produced".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 4,
            },
            HttpParameterSpec {
                name: "W_battery_consumed".into(),
                unit_of_measurement: "kWh".into(),
                factor: 0.001,
                class: "energy".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 5,
            },
            HttpParameterSpec {
                name: "SOC".into(),
                unit_of_measurement: "%".into(),
                factor: 1.0,
                class: "battery".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 7,
            },
            HttpParameterSpec {
                name: "T_batt".into(),
                unit_of_measurement: "°C".into(),
                factor: 0.1,
                class: "temperature".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 8,
            },
            HttpParameterSpec {
                name: "T_batt_module".into(),
                unit_of_measurement: "°C".into(),
                factor: 0.1,
                class: "temperature".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 9,
            },
            HttpParameterSpec {
                name: "V_charge".into(),
                unit_of_measurement: "V".into(),
                factor: 0.01,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 10,
            },
            HttpParameterSpec {
                name: "I_charge".into(),
                unit_of_measurement: "A".into(),
                factor: 0.01,
                class: "current".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 11,
            },
            HttpParameterSpec {
                name: "V_discharge".into(),
                unit_of_measurement: "V".into(),
                factor: 0.01,
                class: "voltage".into(),
            },
        ),
        (
            HttpIndex {
                module_id: 136,
                param_id: 12,
            },
            HttpParameterSpec {
                name: "I_discharge".into(),
                unit_of_measurement: "A".into(),
                factor: 0.01,
                class: "current".into(),
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();
}
