pub mod homeassistant;

pub trait MqttSinkTrait {
    fn sensor_to_mqtt(&mut self, spec: &super::output::Spec);
    fn log_to_mqtt(&mut self, log: &String);
}

impl<T> super::output::Sink for T
where
    T: MqttSinkTrait,
{
    fn sensor(&mut self, spec: &super::output::Spec) {
        self.sensor_to_mqtt(&spec);
    }

    fn log(&mut self, log: &String) {
        self.log_to_mqtt(&log);
    }
}
