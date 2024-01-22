#!/bin/sh

mkdir -p ./prod_cli
# Naive substitution to napi artifacts for the cli binary.
for filename in prod/package/*/*
do
  # 删除 prod/package/ 及其之前的部分
  BINDING_NAME=${filename#*prod/package/}
  # 删除第一个斜杠及其之后的部分
  BINDING_ABI=${BINDING_NAME%%/*}
  BINARY_PATH=${filename%%.*}


  echo "-------------------开始准备变量-------------------"
  echo "文件名 $filename"
  echo "绑定名 $BINDING_NAME"
  echo "ABI 名称 $BINDING_NAME"
  echo "二进制文件路径 $BINARY_PATH"
  echo "-------------------变量准备完成-------------------"

  if [ -f "$BINARY_PATH" ]; then
      chmod +x $BINARY_PATH
      mv -v $BINARY_PATH ./prod_cli/swc-$BINDING_ABI
  elif [ -f "$BINARY_PATH.exe" ]; then
      mv -v $BINARY_PATH.exe ./prod_cli/swc-$BINDING_ABI.exe
  fi
done