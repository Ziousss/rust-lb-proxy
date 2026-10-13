#!/bin/bash

# This file gets rid of the left over from previous runs.

# Import config.sh
source "$(dirname "$0")/config.sh"

if [ "$EUID" -ne 0 ]; then
  echo "Run as root: sudo $0"
  exit 1
fi

for entry in "${ROLES[@]}"; do
    name="${entry%%:*}"

    ip netns delete "$name" 2>/dev/null || true
    ip link delete "veth-$name" 2>/dev/null || true
done

ip link delete "$BRIDGE" 2>/dev/null || true

echo "Takedown completed."