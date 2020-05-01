# vanish-cursor by Alexander Strickner
This small and lightweight tool allows you to vanish the cursor on your screen while using the touchscreen on your linux device. It's aimed for Surface devices, but it should also work for any kind of convertible. 

## Preperation
Before using the tool, you have to install unclutter. On Ubuntu/Debian-based distros you can do that by typing
`sudo apt install unclutter`

## Installation of the tool
To install the tool you can either build it yourself with `cargo` or download the rebuilt version under the release tab of GitHub [here](https://github.com/alexStrickner00/linux_surface_cursor_vanisher/releases). Put the tool anywhere on your device, maybe somewhere, where it is in the PATH.

## Finding the correct input events
To find the correct input events, type `cat /proc/bus/input/devices` into a command line and look for the entry for the touchscreen. In this entry you have to find the _Handlers_ row, which contains the name of the event. The same procedure has to be done to find the event for the touchpad or mouse. 

## Running the tool
After you found the correct events, you can try to run the tool.

The correct syntax is:
`sudo vanish-cursor /dev/input/eventX /dev/input/eventY`, where X is the number of the touchscreen event and Y the number of the mouse/touchpad event.