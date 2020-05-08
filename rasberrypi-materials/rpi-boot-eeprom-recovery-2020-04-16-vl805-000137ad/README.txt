Raspberry Pi 4 EEPROM bootloader rescue image
********************************************

The Raspberry Pi4 contains a small EEPROM used to contain the bootloader.
Normally, this is completely invisible but it it were to become corrupted
(e.g. due to a failed EEPROM upgrade or a broken image or incorrect usage
of flashrom) then the Pi4 will fail to boot.

This rescue image reverts the EEPROM to factory default settings.

This rescue image also updates the USB 3.0 (VL805) EEPROM to the latest
version (137ad)  which reduces power consumption and fixes some issues
with USB webcams.

To re-flash the EEPROM

1. Unzip the contents of this zip file to a blank FAT formatted SD-SDCARD.
2. Power off the Raspberry Pi
3. Insert the sd-card.
4. Power on Raspberry Pi
5. Wait at least 10 seconds.

This easiest method for creating and formatting the SD-CARD is to use the
Raspberry Pi Imager from https://raspberrypi.org/downloads

If successful, the green LED light will blink rapidly (forever), otherwise
an error pattern will be displayed.

If a HDMI display is attached then screen will display green for success
or red if failure a failure occurs.

N.B. This image is not a bootloader it simply replaces the on-board bootloader.
