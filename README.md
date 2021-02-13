# prpd

prpd is a software and hardware solution to make monitoring information from the
NEDAP PowerRouter available although their cloud service was shot down
at 2021-01-31.

The Methods were tested with NEDAP PowerRouter `PR37Bi` Version `7.1.2`.


## Getting the data from the PowerRouter

### HTTP

The PowerRouter normally sends monitoring data every minute via a HTTP-POST
request to `logging1.PowerRouter.com`. This is done without any encryption, so
we can simple bend the network traffic into our direction and read the data.

[More Information](doc/http.md)

### RS485

Some of the data is collected via a RS485 interface of EM24 Energy Sensors by
Carlo Gavazzi. This data is transfered about every second. As the PowerRouter
is already a master on the bus we don't want to disturb any communication and
simple tap into the data lines and listen to the communication.

[More Information](doc/serial.md)

#### Hardware

In order to tab into the RS485 lines, one could access the EM24 Sensors and add
some wires there, but the Sensors are in the fuse box and one doesn't mess with
this one normally.

There is also the need for additional hardware because most of the linux boards
don't support RS485 out of the box. A normal RS485 USB-dongle may be used.

On the other hand, most linux boards do have UART exposed on some headers, so
we can add a UART - RS485 transceiver. The SP3485 has also the handy feature
that it can be configured in a way that it is not possible to write to the
RS485 bus, an ideal feature for our application.

[More Information](hardware/README.md)

## Use the data from the PowerRouter

prpw is written in a way that it should be possible to add you favourite
output. Currently the following outputs are available:

### MQTT Home Assistant

Outputs the data on MQTT so it is possible for Home Assistant to auto discover
all available Data.

### OpenWB

Output the data on MQTT so OpenWB can read the data. This is not done yet, but
will be definitely implemented in the near future.

### Prometheus

With this output it's possible to scrape the data with prometheus.


## Configuration

Configuration is done via environment variables.

* `PRPD_ACTION`: how to obtain the data. either `serial` or `http`. If you want
  to get data from http and serial, you have to run the programm twice.
* `PRPD_OUTPUT_HASS_ACTIVE`: `1` if you want to activate the homeassistant output
* `PRPD_OUTPUT_HASS_MQTT_URI`: mqtt broker for homeassistant output, defaults to
  `tcp://localhost:1883`
* `PRPD_OUTPUT_PROM_ACTIVE`: `1` if you want to activate the prometheus output
* `PRPD_OUTPUT_PROM_PORT`: sets the port of the prometheus output. If you want
  to run serial and http in parallel, you have to specify two different ports
  and configure two differen datasources. in prometheus.
* `PRPD_SERIAL_PORT`: serial port (like `/dev/ttyUSB0`) where to listen for
  data

## Development

### Installing rust and build dependencies

```
# installing rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # https://www.rust-lang.org/tools/install
source $HOME/.cargo/env
rustup toolchain install nightly
rustup default nightly
# installing build dependencies
apt-get install pkg-config libudev-dev cmake
# install target support (only if you want to run this on GL-AR150)
rustup target add mips-unknown-linux-musl
# installing tool to compress executable (only if you want to run this on GL-AR150)
apt-get install upx
```

### Testing locally

```bash
bash dev/run_http.sh # to start the http server
bash dev/http_post_log.sh # to send some test data to the server

bash dev/serial_socat.sh # to create a virtual serial port
bash dev/run_serial.sh # to start prpd in serial mode
python3 dev/serial_write_data.py # to send data to virtual serial port
```

## GL-AR150

GL-AR150 is a cheap, low power network device that can be used as a platform
for prpd. The idea is to use a low power device to gather the data and then
send it to a more powerful device in order to store the data.

[More Information](hardware/GL-AR150.md)


## Disclaimer

Use at your own risk!
