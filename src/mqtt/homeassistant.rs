use paho_mqtt;
use paho_mqtt::message::Message;
use serde;
use serde::Serialize;

use std::collections::HashSet;
use std::env;
use std::time::Duration;

use super::super::output::{Sink, Spec};
use crate::output::{UnitOfMeasurement, Subject};

fn is_str_none(value: &String) -> bool {
    return value == "None";
}

#[derive(Serialize)]
pub struct HassState {
    pub value: f64,
}

#[derive(Serialize)]
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
    client: paho_mqtt::Client,
}

impl MqttSinkHomeAssistant {
    pub fn new() -> MqttSinkHomeAssistant {
        let uri = env::var("PRPD_MQTT_HASS_URI").unwrap_or_else(|_| "tcp://localhost:1883".to_string());
        let mut client = paho_mqtt::Client::new(uri).expect("Error creating the client");
        client.set_timeout(Duration::from_secs(5));

        client.connect(None).expect("Unable to connect");

        return MqttSinkHomeAssistant {
            config_sent: HashSet::new(),
            client,
        };
    }
}

impl super::MqttSinkTrait for MqttSinkHomeAssistant {
    fn get_mqtt_client<'a>(&'a self) -> &'a paho_mqtt::Client {
        return &self.client;
    }

    fn log_to_mqtt(&mut self, log: &String) -> Vec<Message> {
        let mut v = Vec::new();
        v.push(
            paho_mqtt::MessageBuilder::new()
                .topic("prpd/logs")
                .payload(log.as_bytes())
                .qos(1)
                .finalize(),
        );
        return v
    }

    fn sensor_to_mqtt(&mut self, spec: &super::super::output::Spec) -> Vec<Message> {
    //fn to_mqtt(&mut self, spec: &Spec) -> Vec<paho_mqtt::Message> {
        let mut v = Vec::new();

        let topic_prefix = format!("homeassistant/sensor/1/{}", spec.uid);

        if !self.config_sent.contains(spec.uid) {

            // https://www.home-assistant.io/integrations/sensor/#device-class
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
            v.push(
                paho_mqtt::MessageBuilder::new()
                    .topic(format!("{}/config", topic_prefix))
                    .payload(serde_json::to_string(&data).unwrap())
                    .retained(true)
                    .qos(1)
                    .finalize(),
            );
            self.config_sent.insert(spec.uid.clone());
        }
        v.push(
            paho_mqtt::MessageBuilder::new()
                .topic(format!("{}/state", topic_prefix))
                .payload(serde_json::to_string(&HassState { value: spec.value }).unwrap())
                .qos(1)
                .finalize(),
        );
        return v;
    }
}
