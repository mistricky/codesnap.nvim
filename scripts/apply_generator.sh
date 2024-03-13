uname_output=`uname`

function apply_generator {
  if [ -f lua/generator.so ]; then
    rm lua/generator.so
  fi

  cp lua/"${1}generator.so" lua/generator.so
}

if [[ $uname_output == "Darwin" ]]; then
  uname_m_output=`uname -m`

  if [[ $uname_m_output == "arm64" ]]; then
      apply_generator "mac-aarch64"
  elif [[ $uname_m_output == "x86_64" ]]; then
      apply_generator "mac-x86_64"
  else
    echo "Unknown sillicon"
  fi
elif [[ $uname_output == "Linux" ]]; then
  apply_generator "linux-x86_64"
else
  echo "Unknown platform"
fi
