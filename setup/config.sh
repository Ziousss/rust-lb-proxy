#!/bin/bash

BRIDGE="br0"
SUBNET="10.0.0.0/24"

ROLES=(
    "client:10.0.0.10"
    "proxy:10.0.0.20"
    "backend1:10.0.0.30"
    "backend2:10.0.0.31"
    "backend3:10.0.0.32"
)