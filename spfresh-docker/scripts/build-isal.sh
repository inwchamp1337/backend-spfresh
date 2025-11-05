#!/bin/bash
set -euo pipefail

# Build ISAL (isa-l_crypto) following typical cmake build with gcc-9

# Ensure ISAL is available; clone if not found as submodule
if [ ! -d "SPFresh/ThirdParty/isal-l_crypto" ]; then
    echo "ISAL not found in SPFresh/ThirdParty/isal-l_crypto, cloning from https://github.com/intel/isa-l_crypto"
    mkdir -p SPFresh/ThirdParty
    cd SPFresh/ThirdParty
    git clone https://github.com/intel/isa-l_crypto.git isal-l_crypto
    cd ../..
fi

ISAL_DIR="SPFresh/ThirdParty/isal-l_crypto"

echo "Building ISAL in: $ISAL_DIR"
cd "$ISAL_DIR"

# Source gcc setup if present (non-fatal)
if [ -f ../../configs/gcc-setup.sh ]; then
    source ../../configs/gcc-setup.sh || true
elif [ -f ../configs/gcc-setup.sh ]; then
    source ../configs/gcc-setup.sh || true
fi

# Use autotools build as specified
./autogen.sh
./configure
make -j$(nproc)

# Install (optional)
if make -n install >/dev/null 2>&1; then
    echo "Installing ISAL"
    make install
else
    echo "No install target; skipping make install"
fi