#!/usr/bin/env bash

source ./scripts/check_os.sh

function apply_generator {
  cp -rf lua/"${os}generator.so" lua/generator.so
}

apply_generator $os
