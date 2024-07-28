#!/bin/sh

cd ./packages/

for filename in artifacts_rename/*
do
  BINDING_NAME=${filename#*.}
  BINDING_ABI=${BINDING_NAME%%.*}
  CLI_BINARY_PATH=${filename%%.*}

  echo "-------------------开始准备变量-------------------"
  echo "准备上传:"
  echo "文件路径 $filename"
  echo "二进制名称 $BINDING_NAME"
  echo "文件 ABI 名称 $BINDING_ABI"
  echo "Cli 二进制路径 $CLI_BINARY_PATH"
  echo "-------------------变量准备完成-------------------"


  if [ -f "$CLI_BINARY_PATH" ]; then
      chmod +x $CLI_BINARY_PATH
      gh release upload $RELEASE_VERSION $CLI_BINARY_PATH
  elif [ -f "$CLI_BINARY_PATH.exe" ]; then
      gh release upload $RELEASE_VERSION $CLI_BINARY_PATH.exe
  fi

   gh release upload $filename
done
