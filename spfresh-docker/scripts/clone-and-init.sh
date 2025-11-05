#!/bin/bash

# Clone the SPFresh repository
git clone https://github.com/SPFresh/SPFresh.git

# Navigate into the cloned repository
cd SPFresh

# Initialize and update git submodules
git submodule update --init --recursive