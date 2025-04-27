#!/usr/bin/env nix-shell
#!nix-shell -i bash -p openssh

# Remote host details
REMOTE_HOST="eve@eve.home"
CONTAINER_NAME="homepage"

# SSH into the homelab and restart the container
echo "Connecting to $REMOTE_HOST and restarting container $CONTAINER_NAME..."
ssh -o StrictHostKeyChecking=no "$REMOTE_HOST" "sudo -S systemctl restart docker-$CONTAINER_NAME.service"
