#![feature(proc_macro_hygiene, decl_macro)]
use std::env;

#[macro_use]
extern crate rocket;

mod http;
mod mqtt;
mod serial;

use mqtt::homeassistant::ToMqttHomeAssistant;
use mqtt::MqttSender;

fn main() {
    println!("prpd");

    let mut sender = MqttSender::new();
    sender.add_sink(Box::new(ToMqttHomeAssistant::new()));

    match env::var("PRPD_ACTION") {
        Ok(action) => match action.as_str() {
            "serial" => serial::main(sender),
            "http" => http::main(sender),
            _ => panic!("do not understand action"),
        },
        _ => panic!("$PRPD_ACTION has to be set to 'serial' or 'http'."),
    }
}
