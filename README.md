## Robot car with ATmega328p (Arduino chip), made with :package: avr-hal

Working with Rahix's avr-hal to make a little robot :car:🐯 with 📡.

11/08/2020:
Implemented the sensor and the wheels.
15/08/2020:
The car is rolling but the board is making a peeping sound and needs to be restarted
several times on occasions.

The circuit diagram is the same as [this project](https://create.arduino.cc/projecthub/hda-robotics/project-1-2wd-obstacle-avoiding-robot-390ef8).
<div>
<img src="circuit_diagram.jpg" width="400" />
  </div>

## Stuff:

- Arduino UNO or generic using ATmega328p
- [Servo SG90](https://components101.com/servo-motor-basics-pinout-datasheet)
- [2 DC motors 12V](http://robotechshop.com/shop/robotics/motors/dc-motors/yellow-gearbox-motor/?v=f78a77f631d2)
- [Motordriver with H-bridge L298N](https://howtomechatronics.com/tutorials/arduino/arduino-dc-motor-control-tutorial-l298n-pwm-h-bridge/)
- [Sensor HC-SR04](https://www.amazon.co.uk/dp/B07TKVPPHF/ref=as_li_ss_tl?_encoding=UTF8&psc=1&linkCode=sl1&tag=howtomuk-21&linkId=8faa13eaeab406a33ae606e005699aaf&language=en_GB)
- cables, jumpers, breadboards...

## Get started:

1. Install avrdude.
2. Modify the executable flash_it.sh. It contains those lines:
```
cargo +nightly build

flash on the board with: (with your usb serial and your own elf file)

  avrdude -p atmega328p -c arduino -P /dev/tty.usbserial-14430 -U flash:w:target/avr-atmega328p/debug/avr-example.elf:e
screen /dev/tty.usbserial-14430 57600, if you want to show on the console, if not this can be deleted.
```
3. you can now run ./flash_it.sh and have the car running (hopefully).

TODO:

- find the cause of the bug 🐛!

- Continue to organise in structs/Rusty style (the wheels could be passed in a single struct)

- ~~Implement PWM for servo motors~~

- It would not hurt to re-solder the cables (some done)

<img src="here_comes_tiger_3.gif" width="400" />
