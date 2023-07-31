#!/usr/bin/env bash

# Set the can interface up if its down
CAN_INTERFACE=$(ip link show type can | awk -F: '{print $2}' | awk '{$1=$1;print}' | head -n 1)

/usr/bin/ip link set $CAN_INTERFACE down
/usr/bin/ip link set $CAN_INTERFACE up

/usr/bin/candump -l any