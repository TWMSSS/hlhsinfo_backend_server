#!/bin/bash

SERVICE_NAME="hlhsinfo_backend"
SERVICE_DESCRIPTION="HLHSInfo Backend Server"
INSTALLATION_PATH="/usr/local/bin"
EXECUTABLE_NAME="hlhsinfo_backend_server"
SERVICE_FILE="/etc/systemd/system/$SERVICE_NAME.service"

if [ $# -eq 0 ]; then
    echo "Usage: $0 [install|uninstall|enable|disable]"
    exit 1
fi

if [ "$1" == "install" ]; then
    # Check if the service is already installed
    if [ -f "$SERVICE_FILE" ]; then
        echo "Service is already installed."
        exit 0
    fi

    if ![ command -v "openssl" &>/dev/null ]; then
        echo "openssl not installed. Installing..."
        sudo apt-get install -y openssl
    fi

    # Create the service unit file
    echo "[Unit]
    Description=$SERVICE_DESCRIPTION
    After=network.target

    [Service]
    ExecStart=$INSTALLATION_PATH/$EXECUTABLE_NAME
    Restart=always

    [Install]
    WantedBy=multi-user.target" | sudo tee "$SERVICE_FILE" >/dev/null

    # Reload systemd and enable the service
    sudo cp "$EXECUTABLE_NAME" "$INSTALLATION_PATH/$EXECUTABLE_NAME"

    sudo systemctl daemon-reload
    sudo systemctl enable "$SERVICE_NAME"
    sudo systemctl start "$SERVICE_NAME"
    echo "Service installed and enabled. Service started."

elif [ "$1" == "uninstall" ]; then
    # Check if the service is installed
    if [ ! -f "$SERVICE_FILE" ]; then
        echo "Service is not installed."
        exit 0
    fi

    # Stop and disable the service
    sudo systemctl stop "$SERVICE_NAME"
    sudo systemctl disable "$SERVICE_NAME"

    # Remove the service unit file
    sudo rm "$INSTALLATION_PATH/$EXECUTABLE_NAME"
    sudo rm "$SERVICE_FILE"
    echo "Service uninstalled."

elif [ "$1" == "enable" ]; then
    # Check if the service is already enabled
    if systemctl is-enabled "$SERVICE_NAME" >/dev/null; then
        echo "Service is already enabled."
        exit 0
    fi

    # Enable the service
    sudo systemctl enable "$SERVICE_NAME"
    sudo systemctl start "$SERVICE_NAME"
    echo "Service enabled. Service started."

elif [ "$1" == "disable" ]; then
    # Check if the service is already disabled
    if ! systemctl is-enabled "$SERVICE_NAME" >/dev/null; then
        echo "Service is already disabled."
        exit 0
    fi

    # Stop and disable the service
    sudo systemctl stop "$SERVICE_NAME"
    sudo systemctl disable "$SERVICE_NAME"
    echo "Service disabled."

else
    echo "Invalid option. Usage: $0 [install|uninstall|enable|disable]"
    exit 1
fi