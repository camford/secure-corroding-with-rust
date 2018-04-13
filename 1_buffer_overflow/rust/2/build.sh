#!/bin/sh

LINTLOG="lint.log"

indent() { sed 's/^/\t/'; }

# Detect if this is in a Docker container
if [ -f /.dockerenv ]; then
    echo -e "[\033[32;1m+\033[0m] Building in container";
else
    echo -e "[\033[32;1m+\033[0m] Building locally";
fi

# Indent cargo messages but retain cargo's exit code
LINTOUTPUT=$((cargo +nightly clippy --color always 1> /dev/null) 2>&1)
RESULT=$?

if [ $RESULT -eq 0 ]; then
    cargo build --color always 2>&1 | indent
else
    echo -e "$LINTOUTPUT" | indent
    echo -e "[\033[31;1mx\033[0m] The linter doesn't like your project. See above."
fi
