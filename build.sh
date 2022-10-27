#!/bin/bash
#
# Use: ./build.sh "2 - basic arithmetic"
#
# I Love Jesus Christ and Maria <3
#

# Clear files.
cd 1\ -\ project/project
rm -rf src
rm -rf target
mkdir src
cd ../../

# Copy files.
cd "$1"
cp -R * "../1 - project/project/src/"
cd "../1 - project/project/"

# Build.
cargo run


