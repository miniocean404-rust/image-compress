#!/bin/sh

# 将归档的文件重命名移动到另一个文件夹中
OUTPUT_DIR="./artifacts_rename"
ENTRY_DIR="./artifacts/*/*.node"
PROJECT_BINDING_NAME="image-compress"

mkdir -p ./artifacts_rename

for filename in $ENTRY_DIR
do
  BINDING_NAME=${filename#*.}
  BINDING_ABI=${BINDING_NAME%%.*}
  CLI_BINARY_PATH=${filename%%.*}

  echo "-------------------开始准备变量-------------------"
  echo "准备移动 artifacts:"
  echo "二进制名称 $BINDING_NAME"
  echo "文件名称 $filename"
  echo "Cli 二进制路径 $CLI_BINARY_PATH"
  echo "-------------------变量准备完成-------------------"

  if [ -f "$CLI_BINARY_PATH" ]; then
      chmod +x $CLI_BINARY_PATH
      mv -v $CLI_BINARY_PATH ./$OUTPUT_DIR/$PROJECT_BINDING_NAME-$BINDING_ABI
  elif [ -f "$CLI_BINARY_PATH.exe" ]; then
      mv -v $CLI_BINARY_PATH.exe ./$OUTPUT_DIR/$PROJECT_BINDING_NAME-$BINDING_ABI.exe
  fi
done
