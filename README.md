A suite of programs for testing various `atsamd-hal` features on real hardware during their development.

There are programs for both the [Metro M0 board](https://www.adafruit.com/product/3403) for testing features on SAMx11/21 chips, and programs for the [PyGamer board](https://www.adafruit.com/product/4242) for testing on SAMx51 chips.
Often there will be the same tests written for both platforms.
These will have the same names and strive to re-use as much code as possible via the shared libraries.

The PyGamer has a built-in display, but the Metro M0 test programs use the [Adafruit FeatherWing 128x64 OLED display](https://www.adafruit.com/product/4650).

The main subdirectories are the following:
- `lib` - This contains shared libraries useful for multiple tests.
- `metro` - This contains test programs for the Metro M0 board.
- `pygamer` - This contains test programs for the PyGamer board.

Refer to their documentation for the purpose of each individual test program.