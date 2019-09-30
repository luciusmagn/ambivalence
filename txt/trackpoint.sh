#!/bin/bash
echo 255 > /sys/devices/platform/i8042/serio1/serio2/sensitivity &
echo 255 >  /sys/devices/rmi4-00/rmi4-00.fn03/serio2/sensitivity &
echo 1 > /sys/devices/platform/i8042/serio1/serio2/press_to_select &
echo 1 >  /sys/devices/rmi4-00/rmi4-00.fn03/serio2/press_to_select &
echo 300 > /sys/devices/platform/i8042/serio1/serio2/speed &
echo 300 >  /sys/devices/rmi4-00/rmi4-00.fn03/serio2/speed &

