#!/bin/bash

set -e

# Run the GCC setup script
source /configs/gcc-setup.sh

# Clone the SPFresh repository and initialize submodules
./clone-and-init.sh

# Install dependencies
./install-deps.sh

# Build SPDK
./build-spdk.sh

# Build isal-l_crypto
./build-isal.sh

# Build RocksDB
./build-rocksdb.sh

# Build SPFresh
./build-spfresh.sh

# Start the application (replace with the actual command to run SPFresh)
# We run the process normally (not via exec) so we can print the resolved
# absolute path after it exits. This preserves the original exit code.

# Function to resolve an absolute path robustly
resolve_path() {
	target="$1"
	if command -v readlink >/dev/null 2>&1; then
		readlink -f "$target" || echo "$target"
	else
		# Fallback when readlink -f is not available
		echo "$(cd "$(dirname "$target")" >/dev/null 2>&1 && pwd)/$(basename "$target")"
	fi
}

# Common candidate locations for the built SPFresh executable. Update this
# list if your build places the binary somewhere else.
CANDIDATES=("./build/spfresh" "./build/spfresh-server" "./SPFresh/build/spfresh" "./SPFresh/build/spfresh-server")
APP=""
for p in "${CANDIDATES[@]}"; do
	if [ -x "$p" ]; then
		APP="$p"
		break
	fi
done

if [ -z "$APP" ]; then
	# Fallback placeholder — keep this so behavior is explicit if nothing found
	APP="./path/to/spfresh/executable"
	echo "Warning: SPFresh executable not found in candidate locations. Using: $APP"
fi

# Run the application so we can print the path after it finishes.
"$APP"
EXIT_CODE=$?

ABS_PATH=$(resolve_path "$APP")
echo "SPFresh executable path: $ABS_PATH"

exit $EXIT_CODE