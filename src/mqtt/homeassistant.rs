use paho_mqtt;
use serde;
use serde::Serialize;
use std::collections::HashSet;

use super::Spec;
use super::ToMqtt;

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

pub struct ToMqttHomeAssistant {
    config_sent: HashSet<String>,
}

impl ToMqttHomeAssistant {
    pub fn new() -> ToMqttHomeAssistant {
        return ToMqttHomeAssistant {
            config_sent: HashSet::new(),
        };
    }
}

impl ToMqtt for ToMqttHomeAssistant {
    fn log(&mut self, log: &String) -> Vec<paho_mqtt::Message> {
        let mut v = Vec::new();
        v.push(
            paho_mqtt::MessageBuilder::new()
                .topic("prpd/logs")
                .payload(log.as_bytes())
                .qos(1)
                .finalize(),
        );
        return v;
    }

    fn to_mqtt(&mut self, spec: &Spec) -> Vec<paho_mqtt::Message> {
        let mut v = Vec::new();

        let topic_prefix = format!("homeassistant/sensor/1/{}", spec.uid);

        if !self.config_sent.contains(spec.uid) {
            let data = HassConfig {
                device_class: spec.class,
                name: spec.name,
                unit_of_measurement: spec.unit_of_measurement,
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
