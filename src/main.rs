#![feature(proc_macro_hygiene, decl_macro)]
#![feature(format_args_capture)]
use std::env;

#[macro_use]
extern crate rocket;

mod http;
mod mqtt;
mod serial;
mod output;
mod prometheus;

use mqtt::homeassistant::MqttSinkHomeAssistant;
use crate::prometheus::PrometheusSink;

fn main() {
    println!("prpd");

    let mut output = output::Output::new();
    //output.add_sink(Box::new(MqttSinkHomeAssistant::new()));
    output.add_sink(Box::new(PrometheusSink::new()));

    match env::var("PRPD_ACTION") {
        Ok(action) => match action.as_str() {
            "serial" => serial::main(output),
            "http" => http::main(output),
            _ => panic!("do not understand action"),
        },
        _ => panic!("$PRPD_ACTION has to be set to 'serial' or 'http'."),
    }
}
