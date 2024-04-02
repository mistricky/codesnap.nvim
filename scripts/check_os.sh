#!/usr/bin/env bash
os="Unknown"
uname_output=`uname`

if [[ $uname_output == "Darwin" ]]; then
  uname_m_output=`uname -m`

  if [[ $uname_m_output == "arm64" ]]; then
     os="mac-aarch64"
  elif [[ $uname_m_output == "x86_64" ]]; then
     os="mac-x86_64"
  else
    echo "Unknown silicon"
  fi
elif [[ $uname_output == "FreeBSD" ]]; then
  uname_m_output=`uname -m`

  if [[ $uname_m_output == "arm64" ]]; then
     os="freebsd-aarch64"
  elif [[ $uname_m_output == "amd64" ]]; then
     os="freebsd-x86_64"
  else
    echo "Unknown silicon"
  fi
elif [[ $uname_output == "Linux" ]]; then
  os="linux-x86_64"
else
  echo "Unknown platform"
fi
