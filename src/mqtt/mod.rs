use paho_mqtt::message::Message;
use std::env;
use std::time::Duration;

pub mod homeassistant;

pub struct Spec<'a> {
    pub name: &'a String,
    pub uid: &'a String,
    pub value: f64,
    pub class: &'a String,
    pub unit_of_measurement: &'a String,
}

pub trait ToMqtt {
    // TODO: I think I want a coroutine generator here,..
    fn to_mqtt(&mut self, spec: &Spec) -> Vec<Message>;
    fn log(&mut self, log: &String) -> Vec<Message>;
}

pub struct MqttSender {
    sinks: Vec<Box<dyn ToMqtt + Send + Sync>>,
    client: paho_mqtt::Client,
}

impl MqttSender {
    pub fn new() -> MqttSender {
        // Disconnect from the broker
        // rocket.state::<mqtt:Client>()
        // mqtt_client.disconnect(None).unwrap();

        let uri = env::var("PRPD_MQTT_URI").unwrap_or_else(|_| "tcp://localhost:1883".to_string());
        let mut client = paho_mqtt::Client::new(uri).expect("Error creating the client");
        client.set_timeout(Duration::from_secs(5));

        client.connect(None).expect("Unable to connect");

        return MqttSender {
            sinks: Vec::new(),
            client: client,
        };
    }

    pub fn add_sink(&mut self, to_mqtt: Box<dyn ToMqtt + Send + Sync>) {
        self.sinks.push(to_mqtt);
    }

    pub fn send(&mut self, spec: Spec) {
        for sink in &mut self.sinks {
            for message in sink.to_mqtt(&spec) {
                if let Err(e) = self.client.publish(message) {
                    println!("Error sending message: {:?}", e);
                }
            }
        }
    }

    pub fn log(&mut self, log: String) {
        for sink in &mut self.sinks {
            for message in sink.log(&log) {
                if let Err(e) = self.client.publish(message) {
                    println!("Error sending message: {:?}", e);
                }
            }
        }
    }
}
