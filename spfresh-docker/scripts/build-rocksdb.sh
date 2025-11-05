#!/bin/bash

# Set the build directory for RocksDB
BUILD_DIR="/SPFresh/rocksdb/build"

# Create the build directory
mkdir -p $BUILD_DIR
cd $BUILD_DIR

# Clone the RocksDB repository if it doesn't exist
if [ ! -d "./SPFresh/rocksdb" ]; then
    git clone --recursive https://github.com/PtilopsisL/rocksdb.git ./SPFresh/rocksdb
fi

cd ./SPFresh/rocksdb

mkdir -p build && cd build
cmake -DUSE_RTTI=1 -DWITH_JEMALLOC=1 -DWITH_SNAPPY=1 \
      -DCMAKE_C_COMPILER=gcc-9 -DCMAKE_CXX_COMPILER=g++-9 \
      -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_FLAGS="-fPIC" ..
make -j$(nproc)
make install