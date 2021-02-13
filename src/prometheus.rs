use crate::output::{Sink, Spec};
use rocket::config::{Config, Environment};
use rocket::State;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

// TODO: check if https://github.com/tikv/rust-prometheus/issues/315 is resolved, then we could use
// the prometheus client library,..

type Metrics = HashMap<String, String>;

pub struct PrometheusSink {
    metrics: Arc<Mutex<Metrics>>,
}

impl PrometheusSink {
    pub fn new() -> PrometheusSink {
        debug!("starting prometheus output");
        let metrics = Arc::new(Mutex::new(HashMap::new()));

        let metrics_clone = metrics.clone();
        let port = env::var("PRPD_OUTPUT_PROM_PORT")
            .expect("$PRPD_OUTPUT_PROM_PORT needs to be set")
            .parse::<u16>()
            .expect("can not parse $PRPD_OUTPUT_PROM_PORT");

        thread::spawn(move || {
            let config = Config::build(Environment::Production)
                .address("0.0.0.0")
                .port(port)
                .workers(1)
                .unwrap();

            rocket::custom(config)
                .mount("/", routes![metric])
                .manage(metrics_clone)
                .launch();
        });

        PrometheusSink { metrics }
    }
}

#[get("/metrics")]
fn metric(metrics: State<Arc<Mutex<Metrics>>>) -> String {
    debug!("request on /metrics");
    let mut result = String::new();
    {
        let mut local_metrics = metrics.lock().unwrap();
        for value in local_metrics.values() {
            result.push_str(value);
        }
        local_metrics.clear();
        // TODO: shoud not clear: results should not change if queried or not
        // TODO: do clear: we get http data only every 60 seconds, why pretend to have new data?!
    }
    return result;
}

impl Sink for PrometheusSink {
    fn sensor(&mut self, spec: &Spec) {
        let milliseconds_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let prom_id = format!("prpd_{}", &spec.subject);
        let metric = format!(
            "#TYPE {prom_id} gauge\n{prom_id}{{name=\"{name}\",uid=\"{uid}\",unit=\"{unit}\",source=\"{source}\",phase=\"{phase}\"}} {value} {milliseconds_since_epoch}\n",
             prom_id=&prom_id,
             unit=&spec.unit_of_measurement,
             uid=&spec.uid,
             name=&spec.name,
             value=&spec.value,
             source=&spec.source,
             phase=&spec.phase,
             );
        self.metrics
            .lock()
            .unwrap()
            .insert(spec.uid.to_string(), metric);
    }

    fn log(&mut self, log: &String) {
        // ignoring :-/ could at least count it?!
        warn!("got log '{}' and did not handle it", log);
    }
}
