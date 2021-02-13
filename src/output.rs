#[derive(Clone, Debug)]
pub enum Subject {
    Battery,
    Grid,
    PowerRouter,
    Photovoltaics,
    Platform,
    Local,
    Unknown,
}

#[derive(Clone, Debug)]
pub enum UnitOfMeasurement {
    V,
    A,
    VA,
    W,
    Wh,
    Hz,
    DegreeC,
    Percent,
    Unknown,
}

#[derive(Clone, Debug)]
pub enum Phase {
    L1,
    L2,
    L3,
    Irrelevant,
}

#[derive(Debug)]
pub enum Source {
    Http,
    Serial,
}

#[derive(Debug)]
pub struct Spec<'a> {
    pub name: &'a String,
    pub uid: &'a String,
    pub value: f64,
    pub source: &'a Source,
    pub phase: &'a Phase,
    pub subject: &'a Subject,
    pub unit_of_measurement: &'a UnitOfMeasurement,
}

pub trait Sink {
    fn sensor(&mut self, spec: &Spec);
    fn log(&mut self, log: &String);
}

pub struct Output {
    sinks: Vec<Box<dyn Sink + Send + Sync>>,
}

impl std::fmt::Display for UnitOfMeasurement {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            UnitOfMeasurement::V => "V",
            UnitOfMeasurement::DegreeC => "°C",
            UnitOfMeasurement::W => "W",
            UnitOfMeasurement::VA => "VA",
            UnitOfMeasurement::A => "A",
            UnitOfMeasurement::Wh => "Wh",
            UnitOfMeasurement::Percent => "%",
            UnitOfMeasurement::Hz => "Hz",
            UnitOfMeasurement::Unknown => "-",
        })
    }
}

impl std::fmt::Display for Phase {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            Phase::L1 => "l1",
            Phase::L2 => "l2",
            Phase::L3 => "l3",
            Phase::Irrelevant => "-",
        })
    }
}

impl std::fmt::Display for Source {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            Source::Http => "http",
            Source::Serial => "serial",
        })
    }
}

impl std::fmt::Display for Subject {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            Subject::Battery => "battery",
            Subject::Grid => "grid",
            Subject::PowerRouter => "powerrouter",
            Subject::Photovoltaics => "photovoltaics",
            Subject::Platform => "platform",
            Subject::Local => "local",
            Subject::Unknown => "unknown",
        })
    }
}

impl Output {
    pub fn new() -> Output {
        return Output { sinks: Vec::new() };
    }

    pub fn add_sink(&mut self, sink: Box<dyn Sink + Send + Sync>) {
        self.sinks.push(sink);
    }

    pub fn len(&self) -> usize {
        return self.sinks.len();
    }

    pub fn sensor(&mut self, spec: Spec) {
        trace!("sensor {:?}", spec);
        for sink in &mut self.sinks {
            sink.sensor(&spec);
        }
    }

    pub fn log(&mut self, log: String) {
        trace!("log {:?}", log);
        for sink in &mut self.sinks {
            sink.log(&log);
        }
    }
}
