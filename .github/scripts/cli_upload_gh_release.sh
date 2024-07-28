#!/bin/sh

cd ./packages/

# Naive substitution to napi artifacts for the cli binary.
for filename in artifacts/*
do
  echo "尝试上传 $filename"

  BINDING_NAME=${filename#*.}
  BINDING_ABI=${BINDING_NAME%%.*}
  CLI_BINARY_PATH=${filename%%.*}

  if [ -f "$CLI_BINARY_PATH" ]; then
      chmod +x $CLI_BINARY_PATH
      gh release upload $RELEASE_VERSION $CLI_BINARY_PATH
  elif [ -f "$CLI_BINARY_PATH.exe" ]; then
      gh release upload $RELEASE_VERSION $CLI_BINARY_PATH.exe
  fi
done
