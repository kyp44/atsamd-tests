#!/bin/bash
set -e

for dir in */; do
    if [ -f "$dir/Cargo.toml" ]; then
        (cd "$dir" && $*)
    fi
done
