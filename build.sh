#!/bin/bash
#
# Use: ./build.sh "2 - basic arithmetic"
#
# I Love Jesus Christ and Maria <3
#

clear

# Validation.
if [ "$#" -ne 1 ]; then
    echo "Use: ./build.sh \"2 - basic arithmetic\""
    exit 2
fi

if [ -d "$1" ] 
then
    echo "Build: \"$1\"" 
else
    echo "Directory not found: \"$1\""
    exit 2
fi

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


