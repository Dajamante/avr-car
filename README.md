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

1. Install avrdude. It's the utility to write in the ROM(read only memory) or the EEPROM (electrically erasable programmable read-only memory) of AVR microcontrollers. atmega329p, on the arduino uno is from this familly.


2. Modify the executable flash_it.sh. It contains those lines:
 ```
 set -e cargo +nightly build

 avrdude -p atmega328p -c arduino -P /dev/tty.usbserial-14430 -U flash:w:target/avr-atmega328p/debug/avr-example.elf:e

 screen /dev/tty.usbserial-14430 57600
 ```
 ### Details:
 `set -e` is a bash command that will prevent your board to be flashed if an error is returned by cargo.
 You flash on the board with avrdude with your usb serial and your own elf file. You can get your USB with `s /dev/tty* | grep usb`.
 * `-p` is "partno": the only mandatory option, tells avrdude what type of MCU is connected to the programmer
 * `-c` gives the programmer id from a list (luckily arduino is super common)
 * `-U` : perform a memory operation
 The screen command allows to see the console. `57600` is the baud rate, other established baud rates as `9600 are possible, but then you would need to change the program)

<span>3.</span>  you can now run ./flash_it.sh and have the car running (hopefully).

TODO:~

- find the cause of the bug 🐛!

- ~~Continue to organise in structs/Rusty style (the wheels could be passed in a single struct)~~

- ~~Implement PWM for servo motors~~

- It would not hurt to re-solder the cables (some done)

- ~~move Sensor unit in sensor:: namespace~~

- ~~destructure wheels~~

- ~~make a `set e` for the flash.sh file~~

<img src="here_comes_tiger_3.gif" width="400" />
