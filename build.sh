#!/usr/bin/env nix-shell
#!nix-shell -i bash -p docker nix

docker build -t registry.eve.home/homepage:latest .

echo "push with docker push registry.eve.home/homepage:latest"
