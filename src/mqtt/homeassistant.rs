use paho_mqtt;
use paho_mqtt::message::Message;
use serde;
use serde::Serialize;

use std::collections::HashSet;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::super::output::{Sink, Spec};
use crate::output::{Subject, UnitOfMeasurement};

fn is_str_none(value: &String) -> bool {
    return value == "None";
}

#[derive(Serialize)]
pub struct HassState {
    pub value: f64,
}

#[derive(Serialize, Debug)]
struct HassConfig<'a> {
    #[serde(skip_serializing_if = "is_str_none")]
    pub device_class: &'a String, // https://www.home-assistant.io/integrations/sensor/#device-class
    pub name: &'a String,
    pub unit_of_measurement: &'a String,
    pub state_topic: &'a String,
    pub value_template: &'a String,
}

pub struct MqttSinkHomeAssistant {
    config_sent: HashSet<String>,
    client: Arc<Mutex<paho_mqtt::Client>>,
}

impl MqttSinkHomeAssistant {
    pub fn new() -> MqttSinkHomeAssistant {
        let uri = env::var("PRPD_OUTPUT_HASS_MQTT_URI")
            .unwrap_or_else(|_| "tcp://localhost:1883".to_string());
        info!("opening mqtt connection to '{}'", uri);

        let mut client = paho_mqtt::Client::new(uri).expect("Error creating the client");
        client.set_timeout(Duration::from_secs(5));

        let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .connect_timeout(Duration::from_secs(1))
            .retry_interval(Duration::from_secs(1))
            .clean_session(true)
            .finalize();

        client.connect(conn_opts).expect("MQTT: Unable to connect");

        let client_arc = Arc::new(Mutex::new(client));

        let client_arc_clone = client_arc.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            let client = client_arc_clone.lock().unwrap();
            if !client.is_connected() {
                warn!("mqtt disconnected, trying to reconnect");
                match client.reconnect() {
                    Err(e) => error!("MQTT: can not reconnect {}", e),
                    Ok(_) => {}
                }
            }
        });

        return MqttSinkHomeAssistant {
            config_sent: HashSet::new(),
            client: client_arc,
        };
    }
}

impl super::MqttSinkTrait for MqttSinkHomeAssistant {
    fn log_to_mqtt(&mut self, log: &String) {
        self.client
            .lock()
            .unwrap()
            .publish(
                paho_mqtt::MessageBuilder::new()
                    .topic("prpd/logs")
                    .payload(log.as_bytes())
                    .qos(1)
                    .finalize(),
            )
            .unwrap_or_else(|e| error!("Can not send {}", e));
    }

    fn sensor_to_mqtt(&mut self, spec: &super::super::output::Spec) {
        let topic_prefix = format!("homeassistant/sensor/1/{}", spec.uid);

        if !self.config_sent.contains(spec.uid) {
            // https://www.home-assistant.io/integrations/sensor/#device-class
            debug!("config of '{}' was not sent yet, sending it now", spec.uid);
            let device_class = match spec.unit_of_measurement {
                UnitOfMeasurement::DegreeC => "temperature",
                UnitOfMeasurement::W => "power",
                UnitOfMeasurement::VA => "power",
                UnitOfMeasurement::A => "current",
                UnitOfMeasurement::Wh => "energy",
                UnitOfMeasurement::V => "voltage",
                UnitOfMeasurement::Percent => match spec.subject {
                    Subject::Battery => "battery",
                    _ => "None",
                },
                _ => "None",
            };

            let data = HassConfig {
                device_class: &device_class.into(),
                name: spec.name,
                unit_of_measurement: &format!("{}", &spec.unit_of_measurement),
                state_topic: &format!("{}/state", topic_prefix),
                value_template: &"{{ value_json.value}}".into(),
            };
            debug!("config {:?}", data);
            self.client
                .lock()
                .unwrap()
                .publish(
                    paho_mqtt::MessageBuilder::new()
                        .topic(format!("{}/config", topic_prefix))
                        .payload(serde_json::to_string(&data).unwrap())
                        .retained(true)
                        .qos(1)
                        .finalize(),
                )
                .unwrap_or_else(|e| error!("Can not send {}", e));
            self.config_sent.insert(spec.uid.clone());
        }
        self.client
            .lock()
            .unwrap()
            .publish(
                paho_mqtt::MessageBuilder::new()
                    .topic(format!("{}/state", topic_prefix))
                    .payload(serde_json::to_string(&HassState { value: spec.value }).unwrap())
                    .qos(1)
                    .finalize(),
            )
            .unwrap_or_else(|e| error!("Can not send {}", e));
    }
}
