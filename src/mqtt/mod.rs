use paho_mqtt::message::Message;

pub mod homeassistant;

pub trait MqttSinkTrait {
    fn sensor_to_mqtt(&mut self, spec: &super::output::Spec) -> Vec<Message>;
    fn log_to_mqtt(&mut self, log: &String) -> Vec<Message>;
    fn get_mqtt_client<'a>(&'a self) -> &'a paho_mqtt::Client;
}

impl<T> super::output::Sink for T
where
    T: MqttSinkTrait,
{
    fn sensor(&mut self, spec: &super::output::Spec) {
        for message in self.sensor_to_mqtt(&spec) {
            if let Err(e) = self.get_mqtt_client().publish(message) {
                println!("Error sending message: {:?}", e);
            }
        }
    }

    fn log(&mut self, log: &String) {
        for message in self.log_to_mqtt(&log) {
            if let Err(e) = self.get_mqtt_client().publish(message) {
                println!("Error sending message: {:?}", e);
            }
        }
    }
}
