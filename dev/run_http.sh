export PRPD_ACTION=http

export PRPD_OUTPUT_HASS_ACTIVE=1
export PRPD_OUTPUT_HASS_MQTT_URI=tcp://127.0.0.1:1883/

export PRPD_OUTPUT_PROM_ACTIVE=1
export PRPD_OUTPUT_PROM_PORT=8091

export RUST_LOG=prpd=trace,warn
cargo run
