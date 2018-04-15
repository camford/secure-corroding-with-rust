#!/bin/sh

# Detect if this is in a Docker container
if [ -f /.dockerenv ]; then
    echo -e "[\033[32;1m+\033[0m] Building in container";
else
    echo -e "[\033[32;1m+\033[0m] Building locally";
fi
echo

cargo build
