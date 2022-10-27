#!/bin/bash
#
# Use: ./build.sh "2 - basic arithmetic"
#
# I Love Jesus Christ and Maria <3
#

# Clear files.
cd 1\ -\ project/binaries/
rm -rf *
cd ../project/src
rm -rf *
cd ..
rm -rf target

# Copy files.
cd ../../
cd "$1"

cp -R * "../1 - project/project/src/"
cd "../1 - project/project/"

# Build.
cargo run


