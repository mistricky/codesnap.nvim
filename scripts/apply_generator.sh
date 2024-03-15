source ./scripts/check_os.sh

function apply_generator {
  if [ -f lua/generator.so ]; then
    rm lua/generator.so
  fi

  cp lua/"${1}generator.so" lua/generator.so
}

apply_generator $os
