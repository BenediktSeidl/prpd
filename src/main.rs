#![feature(proc_macro_hygiene, decl_macro)]
#![feature(format_args_capture)]
use std::env;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

mod http;
mod mqtt;
mod output;
mod prometheus;
mod serial;

use crate::prometheus::PrometheusSink;
use mqtt::homeassistant::MqttSinkHomeAssistant;

fn main() {
    env_logger::init();
    info!("prpd starting up");

    let mut output = output::Output::new();
    if env::var("PRPD_OUTPUT_HASS_ACTIVE").unwrap_or_else(|_| "".into()) == "1" {
        info!("adding homeassistant output");
        output.add_sink(Box::new(MqttSinkHomeAssistant::new()));
    }
    if env::var("PRPD_OUTPUT_PROM_ACTIVE").unwrap_or_else(|_| "".into()) == "1" {
        info!("adding prometheus output");
        output.add_sink(Box::new(PrometheusSink::new()));
    }

    if output.len() == 0 {
        panic!("no output configured");
    }

    match env::var("PRPD_ACTION") {
        Ok(action) => match action.as_str() {
            "serial" => {
                info!("starting serial input");
                serial::main(output);
            },
            "http" => {
                info!("starting http input");
                http::main(output);
            },
            _ => panic!("do not understand $PRPD_ACTION"),
        },
        _ => panic!("$PRPD_ACTION has to be set to 'serial' or 'http'."),
    }
}
