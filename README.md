# prpd

prpd is a software and hardware solution to make Monitoring Information from the
NEDAP PowerRouter available via MQTT although their cloud service was shot down
at 2021-01-31.

The Methods were tested with NEDAP PowerRouter `PR37Bi` Version `7.1.2`.

There are two ways to obtain the data:


## HTTP

The PowerRouter normally sends monitoring data every minute via a HTTP-POST
request to `logging1.PowerRouter.com`. This is done without any encryption, so
we can simple bend the network traffic into our direction and read the data.

[More Information](doc/http.md)

## RS485

Some of the data is collected via a RS485 interface of EM24 Energy Sensors by
Carlo Gavazzi. This data is transfered about every second. As the PowerRouter
is already a master on the bus we don't want to disturb any communication and
simple tap into the data lines and listen to the communication.

[More Information](doc/serial.md)

## MQTT Output format

* Homeassistant
* OpenWB
* Raw

### Hardware

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

## Configuration

## Development

### Testing locally

```bash
bash dev/run_http.sh # to start the http server
bash dev/http_post_log.sh # to send some test data to the server

bash dev/serial_socat.sh # to create a virtual serial port
bash dev/run_serial.sh # to start prpd in serial mode
python3 dev/serial_write_data.py # to send data to virtual serial port
```

### GL-AR150

GL-AR150 is a cheap, low power network device that can be used as a platform
for prpd

#### Building

```bash
cd dev
bash download_openwrt_sdk.sh
cd ..
bash dev/build.sh
```
#### Deployment

TODO: init.d
TODO: scp

## Disclaimer

Use at your own risk!

## TODO

* mqtt reconnect: Sat Jan 23 10:22:54 2021 daemon.info prpd[5349]: Error sending message: PahoDescr(-3, "Client disconnected")
* retry when mqtt server not reachable at startup