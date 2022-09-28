# rust-raspi-simple-fan

This is a simple application for your raspberry-pi.

I made it because I didn't have a PWM fan and it was a nice little project to:

* learn how to use a transistor as a switch
* fiddle around with the Pi's GPIO pins
* go a little deeper in rust programming

**Three cli arguments are supported right now:**
* `-t u8` for target temperature in °C, default: 55, eg. `-t 50` sets the target CPU temperature to 50°C
* `-p u8` for pin as their BCM number, default: 14, eg. `-p 15` sets the used GPIO pin to BCM number 15
* `-i u64` for interval in milliseconds, default: 3000, eg. `-i 1000` sets the check interval to 1000 milliseconds or 1 second

**Further information**
* at the target cpu temperature the configured GPIO pin gets set to high
* there is a hysteresis temperature of 5°C. At target temperature minus hysteresis temperature the configured GPIO pin gets set to low
* Example: configured temperature is 55°C. The fan will start when CPU reaches `55°C` and try to cool it to `55-5 = 50°C` until it stops. This should help with the fan stoppen and starting right up again.

## Build target

The `.cargo/config` file includes settings to set the build target to `armv7-unknown-linux-musleabihf`.
In order to compile you need to install the needed tools.