#!/bin/sh

mkdir -p ./prod_cli
# Naive substitution to napi artifacts for the cli binary.
for filename in prod/package/*/*.*
do
  # 删除 prod/package/ 及其之前的部分
  BINDING_NAME=${filename#*prod/package/}
  BINDING_ABI=${BINDING_NAME%%/*}
  CLI_BINARY_PATH=${filename%%.*}

  echo "准备构建 图片压缩:"
  echo "绑定名 $BINDING_NAME"
  echo "文件名 $filename"
  echo "二进制文件路径 $CLI_BINARY_PATH"

  if [ -f "$CLI_BINARY_PATH" ]; then
      chmod +x $CLI_BINARY_PATH
      mv -v $CLI_BINARY_PATH ./prod_cli/swc-$BINDING_ABI
  elif [ -f "$CLI_BINARY_PATH.exe" ]; then
      mv -v $CLI_BINARY_PATH.exe ./prod_cli/swc-$BINDING_ABI.exe
  fi
done
