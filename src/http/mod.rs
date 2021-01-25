use rocket::response::status::Created;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json;
use serde_json::Value;
use std::sync::Mutex;

use super::mqtt::MqttSender;

pub mod data;

extern crate paho_mqtt as mqtt;

pub fn main(mut sender: MqttSender) {
    let spec = data::get_specs();

    rocket::ignite()
        .mount("/", routes![logs_json])
        .mount("/", routes![events_json])
        .manage(Mutex::new(sender))
        .manage(spec)
        .launch();
}

#[post("/events.json", format = "json", data = "<body>")]
fn events_json(body: Json<Value>, sender: State<Mutex<MqttSender>>) -> Created<Json<Value>> {
    sender
        .lock()
        .unwrap()
        .log(serde_json::to_string(&body.into_inner()).unwrap());

    return Created(
        "".to_string(),
        Some(Json(
            serde_json::from_str("{\"next-log-level\":2, \"status\":\"ok\"}").unwrap(),
        )),
    );
}

// DATA = {
//     'header': {
//         'powerrouter_id': '9XXXXXXXXXXXXXX5',
//         'time_send': '2021-01-20T23:11:01+01:00',
//         'version': 3,
//         'period': 60
//     },
//     'module_statuses': [
//         {
//             'module_id': 16,
//             'status': 51475,
//             'version': 1,
//             'param_0': 5001,
//             'param_1': 2285,
//             'param_2': 180,
//             'param_3': 8,
//             'param_4': 13729712,
//             'param_5': 4833676
//         }, {

#[post("/logs.json", format = "json", data = "<body>")]
fn logs_json(
    body: Json<Value>,
    sender: State<Mutex<MqttSender>>,
    specs: State<data::HttpSpecs>,
) -> Created<Json<Value>> {
    let json: Value = body.into_inner();
    println!("{}", serde_json::to_string(&json).unwrap());

    let statuses = &json["module_statuses"];

    for status in statuses.as_array().unwrap() {
        let module_id = status["module_id"].as_i64().unwrap();
        for param_id in 0..15 {
            let param_name = format!("param_{}", param_id);
            if let Some(value) = status[&param_name].as_i64() {
                let uid = &format!("http-module_{}-param_{}", module_id, param_id);
                if let Some(spec) = specs.get(&data::HttpIndex {
                    module_id,
                    param_id,
                }) {
                    sender.lock().unwrap().send(super::mqtt::Spec {
                        uid: uid,
                        value: value as f64 * spec.factor,
                        name: &spec.name,
                        class: &spec.class,
                        unit_of_measurement: &spec.unit_of_measurement,
                    });
                } else {
                    sender.lock().unwrap().send(super::mqtt::Spec {
                        uid: uid,
                        value: value as f64,
                        name: &format!("unknown-{}", uid),
                        class: &"None".into(),
                        unit_of_measurement: &"".into(),
                    });
                }
            } else {
                // we don't know how many param_? keys the object have,
                // so we try to read them until it does not work
                break;
            }
        }
    }

    return Created(
        "".to_string(),
        Some(Json(
            serde_json::from_str("{\"next-log-level\":2, \"status\":\"ok\"}").unwrap(),
        )),
    );
}
