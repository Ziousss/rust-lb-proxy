#!/bin/bash

# This file is here to create a fake network inside one single machine
# This network contains one client, one proxy and three backends. These are all connected to a bridge 

# Exit on error
set -e

# Import config.sh
source "$(dirname "$0")/config.sh"

# This entire bash program has to be run as root 
if [ "$EUID" -ne 0 ]; then
  echo "Run as root: sudo $0"
  exit 1
fi

# Get rid of possible left over from previous runs
bash "$(dirname "$0")/takedown.sh"

# The bridge
ip link add "$BRIDGE" type bridge
ip link set "$BRIDGE" up

# Create the 5 namespaces
for entry in "${ROLES[@]}"; do
    name="${entry%%:*}"
    addr="${entry##*:}"

    ip netns add "$name"
    ip link add "veth-$name" type veth peer name "tmp-$name"
    ip link set "tmp-$name" netns "$name" name eth0
    ip link set "veth-$name" master "$BRIDGE"
    ip -n "$name" addr add "$addr/24" dev eth0
    ip link set "veth-$name" up
    ip -n "$name" link set eth0 up
    ip -n "$name" link set lo up
done

for entry in "${ROLES[@]}"; do
    name="${entry%%:*}"
    addr="${entry##*:}"

    # Test
    [ "$name" = "client" ] && continue
    ip netns exec client ping -c1 -W1 "$addr" > /dev/null
    echo "connection client -> $name ($addr) works"
done

echo "Setup successful."