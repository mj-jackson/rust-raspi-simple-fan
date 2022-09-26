# rust-raspi-simple-fan

This is a simple application for your raspberry-pi.

I made it because I didn't have a PWM fan and it was a nice little project to:

* learn how to use a transistor as a switch
* fiddle around with the Pi's GPIO pins
* go a little deeper in rust programming

The current solution is pretty much hard coded in terms of temperature and GPIO pin.

* GPIO pin **14**
* at cpu temp 55°C the fan starts
* there is a hysteresis temp of 5°C, so the fan will stop when the cpu is at 50°C