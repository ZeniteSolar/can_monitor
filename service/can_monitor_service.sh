#!/usr/bin/env bash

# CAN CONFIG
CAN_BITRATE=500000

# Find the first available CAN interface
CAN_INTERFACE=$(ip link show type can | awk -F: '{print $2}' | awk '{$1=$1;print}' | head -n 1)

# If no CAN interface found, try vcan
if [[ -z $CAN_INTERFACE ]]; then
    # Find the first available vcan interface
    VCAN_INTERFACE=$(ip link show type vcan | awk -F: '{print $2}' | awk '{$1=$1;print}' | head -n 1)

    if [[ -z $VCAN_INTERFACE ]]; then
        echo "No CAN or vcan interface found."
        exit 1
    fi

    CAN_INTERFACE=$VCAN_INTERFACE
fi

# Set the can interface up if its down
if ip link show "$CAN_INTERFACE" | grep -q "state DOWN"; then
    echo "Interface $CAN_INTERFACE is down. Setting link to up..."
    ip link set "$CAN_INTERFACE" up type can bitrate $CAN_BITRATE
    echo "Link for $CAN_INTERFACE is now up."
else
    echo "Interface $CAN_INTERFACE is already up."
fi

echo "Starting in the current terminal..."
env DISPLAY=:0 WEBKIT_DISABLE_COMPOSITING_MODE=1 \
    GDK_BACKEND=x11 \
    NO_AT_BRIDGE=1 \
    can-monitor \
    --log-path /home/zenite \
    -p 100
