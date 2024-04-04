#!/usr/bin/env bash

source ./scripts/check_os.sh

(cd ./generator; cargo build $1)

lib_suffix="so"
target="debug"

if [[ $os =~ "mac" ]]; then
  lib_suffix="dylib"
fi

if [[ $1 =~ "release" ]]; then
  target="release"
fi

cp -rf ./generator/target/$target/libgenerator.$lib_suffix lua/generator.so
