#!/bin/sh

indent() { sed 's/^/\t/'; }

# Detect if this is in a Docker container
if [ -f /.dockerenv ]; then
    echo -e "[\033[32;1m+\033[0m] Building in container";
else
    echo -e "[\033[32;1m+\033[0m] Building locally";
fi

OUTPUT=$(make 2>&1)
RESULT=$?
echo "${OUTPUT}" | indent

if [ $RESULT -eq 0 ]; then
    echo -e "[\033[32;1m✔\033[0m] Build successful";
else
    echo -e "[\033[31;1m✗\033[0m] Build failed"
fi
