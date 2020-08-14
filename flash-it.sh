cargo build
# flash on board
avrdude -p atmega328p -c arduino -P /dev/tty.usbserial-14640 -U flash:w:target/avr-atmega328p/debug/avr-example.elf:e
# show on console with baud rate 57600
screen /dev/tty.usbserial-14640 57600
