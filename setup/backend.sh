#!/bin/bash

BACKEND_DIR="$(cd "$(dirname "$0")/../backends" && pwd)"

for i in 1 2 3; do
    gnome-terminal --tab --title="backend$i" -- bash -c "sudo ip netns exec backend$i python3 -m http.server 8080 --d '$BACKEND_DIR/backend$i'"
done