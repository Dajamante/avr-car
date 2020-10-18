# prevent flashing if error
set -e cargo +nightly build

avrdude -p atmega328p -c arduino -P /dev/tty.usbmodem144201 -U flash:w:target/avr-atmega328p/debug/avr-robot-car.elf:e
# show on console with baud rate 57600
screen /dev/tty.usbmodem144201 57600
