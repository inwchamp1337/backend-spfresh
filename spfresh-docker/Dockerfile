FROM ubuntu:20.04

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV GCC_VERSION=9

# Install required packages
RUN apt-get update && \
    apt-get install -y software-properties-common && \
    add-apt-repository ppa:ubuntu-toolchain-r/test && \
    # enable the 'universe' repository so common -dev packages are available
    add-apt-repository universe && \
    apt-get update && \
    apt-get install -y \
    gcc-${GCC_VERSION} \
    g++-${GCC_VERSION} \
    cmake \
    libjemalloc-dev \
    libsnappy-dev \
    libgflags-dev \
    pkg-config \
    swig \
    libboost-all-dev \
    libtbb-dev \
    libisal-dev \
    # SPDK dependencies (development packages)
    libcunit1-dev \
    libaio-dev \
    libssl-dev \
    libjson-c-dev \
    libcmocka-dev \
    uuid-dev \
    libiscsi-dev \
    libtool \
    # Python (some SPDK scripts expect `python`)
    python3 \
    python3-pip \
    python-is-python3 \
    # python-magic library and libmagic development headers (some SPDK tools)
    python3-magic \
    libmagic-dev \
    # Additional SPDK dependencies
    nasm \
    numactl \
    libnuma-dev \
    libaio-dev \
    libssl-dev \
    libjson-c-dev \
    git \
    python3-configshell-fb \
    python3-pexpect \
    && apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set GCC and G++ alternatives
RUN update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-${GCC_VERSION} 60 --slave /usr/bin/g++ g++ /usr/bin/g++-${GCC_VERSION}

# Clone the SPFresh repository and initialize submodules
COPY scripts/clone-and-init.sh /usr/local/bin/clone-and-init.sh
RUN chmod +x /usr/local/bin/clone-and-init.sh && \
    /usr/local/bin/clone-and-init.sh

# Install dependencies
COPY scripts/install-deps.sh /usr/local/bin/install-deps.sh
RUN chmod +x /usr/local/bin/install-deps.sh && \
    /usr/local/bin/install-deps.sh

# Build SPDK
COPY scripts/build-spdk.sh /usr/local/bin/build-spdk.sh
RUN chmod +x /usr/local/bin/build-spdk.sh && \
    /usr/local/bin/build-spdk.sh

# Build isal-l_crypto
COPY scripts/build-isal.sh /usr/local/bin/build-isal.sh
RUN chmod +x /usr/local/bin/build-isal.sh && \
    /usr/local/bin/build-isal.sh

# Build RocksDB
COPY scripts/build-rocksdb.sh /usr/local/bin/build-rocksdb.sh
RUN chmod +x /usr/local/bin/build-rocksdb.sh && \
    /usr/local/bin/build-rocksdb.sh

# Build SPFresh
COPY scripts/build-spfresh.sh /usr/local/bin/build-spfresh.sh
RUN chmod +x /usr/local/bin/build-spfresh.sh && \
    /usr/local/bin/build-spfresh.sh

# Copy the release artifacts file to the root for easy access
#COPY /build/release-artifacts.txt /release-artifacts.txt

# Copy the collect artifacts script
#COPY scripts/collect-artifacts.sh /usr/local/bin/collect-artifacts.sh
#RUN chmod +x /usr/local/bin/collect-artifacts.sh

# Set the entry point
COPY scripts/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]