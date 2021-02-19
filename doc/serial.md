# RS485 Communication

Official [Installation Manual][manual] *[(archived)](manual_archived)*

[manual]: https://www.axun-solar.com/docs/autoconso_onduleur/EN_1409_Manual_Installation-manual-PowerRouter-Unifit.pdf
[manual_archived]: https://web.archive.org/web/20210124090427/https://www.axun-solar.com/docs/autoconso_onduleur/EN_1409_Manual_Installation-manual-PowerRouter-Unifit.pdf


## Hardware

The two EM24 Sensors are connected with the PowerRouter via RS485 bus. They are
connected via standard CAT-cables. The jack is at the right side of the
PowerRouter and labeled `CAN`. Be careful, it's the one at the backside, the
jack at the front has some LEDs and is for the network communication!

As mentioned in the Installation Manual the Sensors are connected as follow:

```
     [cable] -> [EM24]
 white/green -> 41 (A-)
       green -> 42 (B+)
white/orange -> 43 (GND)
```

The cable seems to be wired by `T568B` thus it appears that the following
pinout is used:

```
[RJ45] -> [RS485]
     3 -> A-
     6 -> B+
     1 -> GND
```

But as explained on [wikipedia](https://en.wikipedia.org/wiki/RS-485#Signals)
the data lines of the transceiver SP3485 is swapped.

```
[RJ45] -> [RS485] -> [SP3485]
     3 ->    A-   -> B (Inverting)
     6 ->    B+   -> A (Non-Inverting)
     1 ->   GND   -> GND
```

## Serial Port Settings

* baud: 9600
* stop bits: one
* parity: none

## Protocol

Sample communication

```
01 03 00 0c 00 0a 05 ce
01 03 14 01 c9 00 00 02 33 00 00 04 d5 00 00 01 42 00 00 04 85 00 00 74 9f
01 03 00 16 00 08 a5 c8
01 03 10 0a 37 00 00 04 0e 00 00 05 07 00 00 0a c4 00 00 24 15
02 03 00 46 00 06 24 2e
02 03 0c 00 00 00 00 00 00 00 00 c9 ad 00 03 3e 0d
01 03 00 0c 00 0a 05 ce
01 03 14 01 c9 00 00 02 33 00 00 04 e1 00 00 01 42 00 00 04 85 00 00 9e af
01 03 00 16 00 08 a5 c8
01 03 10 0a 57 00 00 03 f5 00 00 05 09 00 00 0a e2 00 00 99 da
01 03 00 00 00 06 c5 c8
01 03 0c 08 e1 00 00 08 f2 00 00 08 b7 00 00 41 bf
01 03 00 0c 00 0a 05 ce 
01 03 14 01 be 00 00 02 3b 00 00 04 df 00 00 01 12 00 00 04 85 00 00 e2 fb
01 03 00 16 00 08 a5 c8
01 03 10 0a 4d 00 00 03 f5 00 00 05 1b 00 00 0a de 00 00 f1 dc 
01 03 00 46 00 06 24 1d
01 03 0c 2e 7b 00 01 bb 15 00 00 a3 0f 00 00 65 2c
01 03 00 0c 00 0a 05 ce
01 03 14 01 c1 00 00 02 31 00 00 04 f5 00 00 00 f5 00 00 04 85 00 00 b9 f8
01 03 00 16 00 08 a5 c8
01 03 10 0a 76 00 00 03 fd 00 00 05 04 00 00 0b 0f 00 00 1f ab
02 03 00 0c 00 0a 05 fd
02 03 14 00 00 00 00 00 29 00 00 00 42 00 00 00 00 00 00 ff e1 ff ff 21 77
01 03 00 0c 00 0a 05 ce
01 03 14 01 bf 00 00 02 35 00 00 04 f5 00 00 01 04 00 00 04 8c 00 00 7c d8
01 03 00 16 00 08 a5 c8
01 03 10 0a 43 00 00 03 f9 00 00 05 0d 00 00 0a cd 00 00 c6 8c
02 03 00 16 00 08 a5 fb
02 03 10 ff f9 ff ff 00 00 00 00 00 5d 00 00 00 93 00 00 4f 06
01 03 00 0c 00 0a 05 ce
01 03 14 01 db 00 00 02 35 00 00 04 d8 00 00 00 e9 00 00 04 80 00 00 bf 4d
01 03 00 16 00 08 a5 c8
01 03 10 0a 43 00 00 04 39 00 00 05 0d 00 00 0a cd 00 00 cf 44
02 03 00 46 00 06 24 2e
02 03 0c 00 00 00 00 00 00 00 00 c9 ad 00 03 3e 0d
01 03 00 0c 00 0a 05 ce
01 03 14 01 c4 00 00 02 35 00 00 04 d8 00 00 01 4b 00 00 04 80 00 00 6e 9d
01 03 00 16 00 08 a5 c8
01 03 10 0a 44 00 00 04 00 00 00 05 0d 00 00 0a cd 00 00 59 da
```

The hexdump above shows the requests of the PowerRouter and the response of the
EM24-DIN. The request is structured as following:

```
01 03 00 16 00 08 a5 c8
                  ^^^^^ crc (bytes are swapped!)
            ^^^^^------ quantity of parameters to read
      ^^^^^------------ start address
   ^^------------------ function code: 03: read holding registers
^^--------------------- device address
```

The response:

```
01 03 10 0a 37 00 00 04 0e 00 00 05 07 00 00 0a c4 00 00 24 15
                                                         ^^^^^ crc (bytes are swapped!)
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------ data
      ^^------------------------------------------------------ bytes transfered
   ^^--------------------------------------------------------- function code
^^------------------------------------------------------------ device address
```


The communication protocol and data can be decoded with the help of the official
[EM24-DIN Communication Protocol Manual][em24_manual]
*[(archived)][em24_manual_archived]* of the energy meter.

[em24_manual]: https://www.ccontrols.com/support/dp/CarloGavazziEM24.pdf
[em24_manual_archived]: https://web.archive.org/web/20210124095047/https://www.ccontrols.com/support/dp/CarloGavazziEM24.pdf
