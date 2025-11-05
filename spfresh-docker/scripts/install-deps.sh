#!/bin/bash

set -e

# Update package list and install necessary dependencies
apt-get update && apt-get install -y \
    cmake \
    libjemalloc-dev \
    libsnappy-dev \
    libgflags-dev \
    pkg-config \
    swig \
    libboost-all-dev \
    libtbb-dev \
    libisal-dev \
    libcunit1-dev \
    libaio-dev \
    libssl-dev \
    libjson-c-dev \
    libcmocka-dev \
    uuid-dev \
    libiscsi-dev \
    python-is-python3 \
    libncurses5-dev \
    libncursesw5-dev \
    help2man \
    systemtap-sdt-dev
# Clean up
apt-get clean && rm -rf /var/lib/apt/lists/*