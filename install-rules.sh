#!/usr/bin/sh

if [ -f "$(pwd)/Cargo.toml" ]; then
  project_name="$(bash -c 'cat Cargo.toml | head -2 | tail -1')"
  if ! [ "$project_name" = "name = \"cruster\"" ]; then
    echo "Script was luanched from the dir of project '$project_name'. But you must run this script from cruster project dir."
    echo "Try: 'git clone https://github.com/sinKettu/cruster && cd cruster && bash ./install-rules.sh'"
    exit 1
  fi
else
    echo "Cannot find 'Cargo.toml' in current dir. You must run this script from cruster project dir."
    echo "Try: 'git clone https://github.com/sinKettu/cruster && cd cruster && bash ./install-rules.sh'"
    exit 1
fi

rules_dir="$(pwd)/rules"
if ! [ -d "$rules_dir" ]; then
    echo "'rules' directory is not found at current location; please, run the script from Cruster directory.";
    exit 1;
fi;

mkdir -p "~/.cruster"
cp -v -r "$rules_dir" "${cruster_base_dir}/"

if [ "$?" = "0" ]; then
    echo "Rules are installed successfully into '${cruster_base_dir}/rules'";
else
    echo "There were issues while installing rules. See above.";
fi;
