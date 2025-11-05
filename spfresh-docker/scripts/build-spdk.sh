#!/bin/bash
set -euo pipefail

# Build SPDK following the requested commands:
# cd ThirdParty/spdk
# ./scripts/pkgdep.sh
# CC=gcc-9 ./configure
# CC=gcc-9 make -j$(nproc)


# Search common locations for the SPDK tree. The clone script places the repo in
# ./SPFresh, so prefer paths under that. Fall back to top-level ThirdParty/spdk
# or spdk in case the layout is different.
if [ -d "SPFresh/ThirdParty/spdk" ]; then
	SPDK_DIR="SPFresh/ThirdParty/spdk"
elif [ -d "SPFresh/spdk" ]; then
	SPDK_DIR="SPFresh/spdk"
elif [ -d "ThirdParty/spdk" ]; then
	SPDK_DIR="ThirdParty/spdk"
elif [ -d "spdk" ]; then
	SPDK_DIR="spdk"
else
	echo "ERROR: SPDK directory not found (tried SPFresh/ThirdParty/spdk, SPFresh/spdk, ThirdParty/spdk, spdk)"
	exit 1
fi

echo "Building SPDK in: $SPDK_DIR"
cd "$SPDK_DIR"

# Source gcc setup if present (non-fatal)
if [ -f ../../configs/gcc-setup.sh ]; then
	# When this script is run from project root -> SPDK in ThirdParty/spdk, the configs file is two levels up
	source ../../configs/gcc-setup.sh || true
elif [ -f ../configs/gcc-setup.sh ]; then
	source ../configs/gcc-setup.sh || true
fi

# Run package dependency installer if present
if [ -x ./scripts/pkgdep.sh ]; then
	echo "Running ./scripts/pkgdep.sh"

	# Some pkgdep.sh scripts call 'python' (the package name) which is not
	# available on Ubuntu 20.04; ensure the script uses python3 instead so
	# apt won't try to install the obsolete 'python' package during build.
	if command -v sed >/dev/null 2>&1; then
		# Make an in-place replacement of standalone 'python' tokens to 'python3'.
		# This is conservative and only targets whole-word occurrences.
		sed -E -i.bak 's/\bpython\b/python3/g' ./scripts/pkgdep.sh || true
		# Also replace pip package names that are incorrect for Ubuntu 20.04
		sed -E -i.bak 's/\bpython3-magic\b/python-magic/g' ./scripts/pkgdep.sh || true
		echo "Patched ./scripts/pkgdep.sh to prefer python3 and correct pip package names (backup at ./scripts/pkgdep.sh.bak)"
	fi

	# Ensure python-magic is available for scripts that expect it. Try system
	# package first (installed via apt in the Dockerfile); then install the
	# pip package 'python-magic' as a fallback so pip-based installs succeed.
	if command -v python3 >/dev/null 2>&1; then
		python3 -m pip install --no-cache-dir python-magic || true
	fi

	# Additionally, patch any other shell scripts under the cloned SPFresh tree
	# that might reference the 'python' package so pkgdep won't attempt to
	# install the obsolete 'python' package. This is conservative and only
	# edits *.sh files (backups saved with .bak extension).
	if command -v grep >/dev/null 2>&1 && [ -d "../.." ]; then
		ROOT_DIR="../.."
		# Find shell scripts under SPFresh and replace whole-word 'python' -> 'python3'
		grep -RIl --exclude-dir=.git --include='*.sh' '\bpython\b' "$ROOT_DIR" || true
		grep -RIl --exclude-dir=.git --include='*.sh' '\bpython\b' "$ROOT_DIR" | xargs -r sed -E -i.bak 's/\bpython\b/python3/g' || true
		# Also replace incorrect pip package names
		grep -RIl --exclude-dir=.git --include='*.sh' '\bpython3-magic\b' "$ROOT_DIR" | xargs -r sed -E -i.bak 's/\bpython3-magic\b/python-magic/g' || true
		echo "Patched shell scripts under $ROOT_DIR to prefer python3 and correct pip package names where found (backups .bak)"
	fi

	./scripts/pkgdep.sh
else
	echo "pkgdep.sh not found or not executable; skipping ./scripts/pkgdep.sh"
fi

# Configure and build using gcc-9 as requested
export CC=gcc-9
echo "Configuring SPDK with CC=$CC"
if [ -x ./configure ]; then
	./configure
else
	echo "No configure script found; skipping configure step"
fi

echo "Running make -j$(nproc) with CC=$CC"
make -j$(nproc)

# Install (optional) — run if Makefile supports it
if make -n install >/dev/null 2>&1; then
	echo "Installing SPDK"
	make install
else
	echo "No install target or install would be no-op; skipping make install"
fi