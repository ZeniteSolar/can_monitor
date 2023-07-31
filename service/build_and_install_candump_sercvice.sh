#!/usr/bin/env bash

# Install dependencies
sudo apt install -y can-utils

# CAN_INTERFACE=$(ip link show type can | awk -F: '{print $2}' | awk '{$1=$1;print}' | head -n 1)
CAN_INTERFACE=can0
CAN_BITRATE=500000

# Setup can kernel modules to load at boot
MODULES_LOAD_SYSTEM_FILE=/etc/modules-load.d/can.conf
MODULES_LOAD_LOCAL_FILE=$PWD/can.conf
cat << EOF > $MODULES_LOAD_LOCAL_FILE
can
can_raw
EOF
sudo ln -sf $MODULES_LOAD_LOCAL_FILE $MODULES_LOAD_SYSTEM_FILE
sudo modprobe can can_raw

# Setup can interface to automatically start on at boot
NETWORK_SYSTEM_FILE=/etc/systemd/network/80-can.network
NETOWRK_LOCAL_FILE=$PWD/80-can.network
cat << EOF > $NETWORK_LOCAL_FILE
[Match]
Name=can*
[CAN]
BitRate=$CAN_BITRATE
RestartSec=100ms
EOF
sudo ln -sf $NETOWRK_LOCAL_FILE $NETWORK_SYSTEM_FILE

# Enable systemd-network and restart it because it may already be running
sudo systemctl enable systemd-networkd
sudo systemctl restart systemd-networkd

# Set the can interface up if its down
if ip link show "$CAN_INTERFACE" | grep -q "state DOWN"; then
    echo "Interface $CAN_INTERFACE is down. Setting link to up..."
    ip link set "$CAN_INTERFACE" up type can bitrate $CAN_BITRATE
    echo "Link for $CAN_INTERFACE is now up."
else
    echo "Interface $CAN_INTERFACE is already up."
fi

# Generate the Can Monitor service file
SERVICE_FILE=$PWD/can_monitor.service
cat << EOF > $SERVICE_FILE
[Unit]
Description=candump
After=can0.service local-fs.target
Requires=can0.service
DefaultDependencies=no

[Service]
Type=simple
ExecStart=/bin/bash -c '$PWD/candump_service.sh'
ExecStopPost=/bin/rm -rf /tmp/candump
Restart=always

[Install]
WantedBy=sysinit.target
EOF

# Install the systemd service
sudo systemctl link $SERVICE_FILE
sudo systemctl daemon-reload
sudo systemctl enable can_monitor.service
