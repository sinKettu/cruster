#!/usr/bin/sh

rules_dir="$(pwd)/rules"
if ! [ -d "$rules_dir" ]; then
    echo "'rules' directory is not found at current location; please, run the script from Cruster directory.";
    exit 1;
fi;

cruster_base_dir="${HOME}/.cruster"
if ! [ -d "$cruster_base_dir" ]; then
    mkdir "$cruster_base_dir";
fi;

cp -r "$rules_dir" "${cruster_base_dir}/rules"

if [ "$?" = "0" ]; then
    echo "Rules are installed successfully into '${cruster_base_dir}/rules'";
else
    echo "There were issues while installing rules. See above.";
fi;
