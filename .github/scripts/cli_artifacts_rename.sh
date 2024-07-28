#!/bin/sh

cd ./packages/

# 将归档的文件重命名移动到另一个文件夹中
ENTRY_DIR="artifacts/*/*.node" # 不能有 ./
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

  if [ -f "$CLI_BINARY_PATH" ]; then
      chmod +x $CLI_BINARY_PATH
      mv -v $CLI_BINARY_PATH $OUTPUT_DIR/$PROJECT_BINDING_NAME-$BINDING_ABI
  elif [ -f "$CLI_BINARY_PATH.exe" ]; then
      mv -v $CLI_BINARY_PATH.exe $OUTPUT_DIR/$PROJECT_BINDING_NAME-$BINDING_ABI.exe
  fi

  echo "-------------------输出-------------------"
  echo "移动前：$CLI_BINARY_PATH"
  echo "移动后：$OUTPUT_DIR/$PROJECT_BINDING_NAME-$BINDING_ABI"
  echo "-------------------输出-------------------"
done
