#!/bin/bash

# Check if a higher version of GCC is installed
if ! command -v gcc-10 &> /dev/null
then
    echo "GCC version 10 or higher is required. Please install it."
    exit 1
fi

# Set the environment variables to use the higher version of GCC
export CC=gcc-10
export CXX=g++-10

echo "GCC version set to $CC and C++ compiler set to $CXX."