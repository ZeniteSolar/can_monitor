#!/usr/bin/env bash

# Generate the Can Monitor service file
SERVICE_FILE=$PWD/can_monitor.service
cat << EOF > $SERVICE_FILE
[Unit]
Description=Can Monitor
After=graphical.target

[Service]
ExecStart=$PWD/run.sh
WorkingDirectory=$PWD
User=$USER
Restart=always
RestartSec=10

[Install]
WantedBy=graphical.target
EOF
echo "Service file '$SERVICE_FILE' generated."

# Install the systemd service
sudo systemctl link $SERVICE_FILE
sudo systemctl daemon-reload
sudo systemctl enable can_monitor.service
