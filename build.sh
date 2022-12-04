#!/bin/bash

# Build cargo libraries
cd src
cargo build

# Ignore these directories
ignore=("target")
dllname=""
# Iterate over directories
for d in *; do
    if [ -d "$d" ] && [[ ! "${ignore[*]}" =~ "${d}" ]]; then
        dllname="${d/-/_}"
        cd ..
        sed "s _X_PATH_X_ ${dllname} g" "$PWD/templates/gdnlib.gdnlib" > "$PWD/bin/$dllname.gdnlib"
        cd src
    fi
done
