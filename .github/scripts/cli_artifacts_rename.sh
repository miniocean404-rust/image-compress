#!/bin/sh

cd ./packages/

# 重命名二进制文件 为 image-compress-<ABI>.<ext>，移动 .node 文件到 artifacts_rename 文件夹
ENTRY_DIR="artifacts/*/image-compress*" # 不能有 ./
OUTPUT_DIR="./artifacts_rename"
PROJECT_BINDING_NAME="image-compress"

mkdir -p $OUTPUT_DIR

for filename in $ENTRY_DIR
do
  BINDING_NAME=${filename#*.}
  # 获取
  BINDING_ABI=${BINDING_NAME%%.*}
  # 获取文件后缀名
  CLI_BINARY_PATH=${filename%%.*}

  echo "-------------------开始准备变量-------------------"
  echo "准备移动 artifacts:"
  echo "文件路径 $filename"
  echo "二进制名称 $BINDING_NAME"
  echo "Cli 二进制路径 $CLI_BINARY_PATH"
  echo "-------------------变量准备完成-------------------"

  # -f 是一个测试操作符，用于测试指定的路径是否为一个常规文件（不是目录、设备文件等）
  if [ -f "$CLI_BINARY_PATH" ]; then
      chmod +x $CLI_BINARY_PATH
      mv -v $CLI_BINARY_PATH        $OUTPUT_DIR/$PROJECT_BINDING_NAME-$BINDING_ABI
  elif [ -f "$CLI_BINARY_PATH.exe" ]; then
      mv -v $CLI_BINARY_PATH.exe    $OUTPUT_DIR/$PROJECT_BINDING_NAME-$BINDING_ABI.exe
  fi

  if [ -f "$filename" ]; then
      mv -v $filename $OUTPUT_DIR
  fi

  ls -R $OUTPUT_DIR
done
