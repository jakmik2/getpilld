#!/bin/bash
class_name=$1
gdlibrary="rustlib"

# Make template in scripts
cp "$PWD/templates/gdns.gdns" "$PWD/scripts/$class_name.gdns"

# Swap out template vars
sed -i "s _X_LIBRARY_X_ ${gdlibrary} g" "$PWD/scripts/$class_name.gdns"
sed -i "s _X_CLASS_X_ ${class_name} g" "$PWD/scripts/$class_name.gdns"
