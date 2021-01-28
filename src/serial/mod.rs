use std::convert::TryInto;
use std::env;
use std::time::Duration;

extern crate crc_any;

use crc_any::CRC;

use super::output::{Output, Phase, Source, Subject, UnitOfMeasurement};

mod data;

pub fn check_crc(data: &Vec<u8>) -> bool {
    let len = data.len();
    let mut crc_modbus = CRC::crc16modbus();
    crc_modbus.digest(&data.as_slice()[..len - 2]);
    let crc = crc_modbus.get_crc_vec_le();
    return data.as_slice()[len - 2..] == crc;
}

pub fn main(mut output: Output) {
    let specs = data::get_specs();

    let port_name = env::var("PRPD_SERIAL_PORT").expect("need to set $PRPD_SERIAL_PORT");

    let mut port = serialport::new(port_name, 9600)
        .timeout(Duration::from_millis(3000))
        .stop_bits(serialport::StopBits::One)
        .parity(serialport::Parity::None)
        .open()
        .expect("Failed to open port");

    loop {
        let mut request: Vec<u8> = vec![0; 8];

        // TODO: no expect, but silent fail or smth...
        port.read_exact(request.as_mut_slice())
            .expect("Found no data!");

        // find the first valid sequence of 8 elements: this is a request!
        while !check_crc(&request) {
            println!("search for start");
            let mut byte: Vec<u8> = vec![0u8; 1];
            port.read_exact(byte.as_mut_slice()).expect("error reading");
            request.rotate_left(1);
            request[7] = byte[0];
        }

        // so we found a valid request, let's decode what we really have:
        let device_address: u8 = request[0];
        let function_code: u8 = request[1];
        let starting_address: u16 = u16::from_be_bytes(request[2..4].try_into().unwrap());
        let quantity: u16 = u16::from_be_bytes(request[4..6].try_into().unwrap());

        let mut response: Vec<u8> = vec![0; (5 + quantity * 2) as usize];
        port.read_exact(response.as_mut_slice())
            .expect("error reading");
        if !check_crc(&response) {
            println!("response crc wrong!");
            continue;
        }

        let len = request.len();
        let response_device_address: u8 = request[0];
        let response_function_code: u8 = request[1];
        //let response_length: u8 = request[2];

        if response_device_address != device_address || response_function_code != function_code {
            println!("response does not match request");
            continue;
        }

        let raw_data = &response.as_slice()[3..response.len() - 2];

        // we have the data, and currently it only consists of 4 bytes, so we can chunk it

        for (i, data) in raw_data.chunks(4).enumerate() {
            let address = starting_address + i as u16 * 2u16;
            // fix word order (this seems to be a modbus thing?!)
            let fixed: [u8; 4] = [data[2], data[3], data[0], data[1]];
            let value = i32::from_be_bytes(fixed) as f64;
            let uid = &format!("device_{}-address_{}", device_address, address);

            let subject = match device_address {
                1 => Subject::Grid,
                2 => Subject::Photovoltaics,
                _ => Subject::Unknown,
            };

            if let Some(spec) = specs.get(&address) {
                output.sensor(super::output::Spec {
                    name: &spec.name,
                    uid: uid,
                    value: value * spec.factor,
                    source: &Source::Serial,
                    phase: &spec.phase,
                    subject: &subject,
                    unit_of_measurement: &spec.unit_of_measurement,
                });
            } else {
                output.sensor(super::output::Spec {
                    name: &format!("unknown-{}", uid),
                    uid: uid,
                    value: value,
                    source: &Source::Serial,
                    phase: &Phase::Irrelevant,
                    subject: &Subject::Unknown,
                    unit_of_measurement: &UnitOfMeasurement::Unknown,
                });
            }
        }
    }
}
