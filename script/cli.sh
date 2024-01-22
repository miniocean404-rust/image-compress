#!/bin/sh

mkdir -p ./prod_cli
# Naive substitution to napi artifacts for the cli binary.
for filename in prod/package/*/*.node
do
  BINDING_NAME=${filename#*.}
  BINDING_ABI=${BINDING_NAME%%.*}
  CLI_BINARY_PATH=${filename%%.*}

  echo "准备构建 artifacts:"
  echo "绑定名 $BINDING_NAME"
  echo "文件名 $filename"
  echo "Cli 二进制路径 $CLI_BINARY_PATH"

  if [ -f "$CLI_BINARY_PATH" ]; then
      chmod +x $CLI_BINARY_PATH
      mv -v $CLI_BINARY_PATH ./prod_cli/swc-$BINDING_ABI
  elif [ -f "$CLI_BINARY_PATH.exe" ]; then
      mv -v $CLI_BINARY_PATH.exe ./prod_cli/swc-$BINDING_ABI.exe
  fi
done
