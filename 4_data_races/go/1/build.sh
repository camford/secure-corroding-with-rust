#!/bin/sh

indent() { sed 's/^/\t/'; }

# Detect if this is in a Docker container
if [ -f /.dockerenv ]; then
    echo -e "[\033[32;1m+\033[0m] Building in container";
else
    echo -e "[\033[32;1m+\033[0m] Building locally";
fi

export GOPATH=`pwd`

# Compile
echo -e "[\033[32;1m+\033[0m] Compiling";
go build -o inc main | indent

# Test
echo -e "[\033[32;1m+\033[0m] Running tests";
TESTOUTPUT=$(go test race_test 2>&1)
RESULT=$?
if [ $RESULT -eq 0 ]; then
    echo -e "[\033[32;1m✔\033[0m] Tests passed";
else
    echo -e "[\033[31;1mx\033[0m] Tests failed"
fi
echo -e "$TESTOUTPUT" | indent

# Data race detection
DRDOUTPUT=$(go test -race race_test 2>&1)
RESULT=$?

if [ $RESULT -eq 0 ]; then
    echo -e "[\033[32;1m✔\033[0m] No data races detected";
else
    echo -e "[\033[31;1m✗\033[0m] Data race detected."
    echo -e "$DRDOUTPUT" | indent
fi
