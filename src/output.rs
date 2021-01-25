pub struct Spec<'a> {
    pub name: &'a String,
    pub uid: &'a String,
    pub value: f64,
    pub class: &'a String,
    pub unit_of_measurement: &'a String,
}

pub trait Sink {
    fn sensor(&mut self, spec: &Spec);
    fn log(&mut self, log: &String);
}

pub struct Output {
    sinks: Vec<Box<dyn Sink + Send + Sync>>,
}

impl Output {
    pub fn new() -> Output {
        return Output {
            sinks: Vec::new(),
        };
    }

    pub fn add_sink(&mut self, sink: Box<dyn Sink + Send + Sync>) {
        self.sinks.push(sink);
    }

    pub fn sensor(&mut self, spec: Spec) {
        for sink in &mut self.sinks {
            sink.sensor(&spec);
        }
    }

    pub fn log(&mut self, log: String) {
        for sink in &mut self.sinks {
            sink.log(&log);
        }
    }
}
