#!/bin/sh

mkdir -p ./prod_cli
# Naive substitution to napi artifacts for the cli binary.

for path in prod/package/*/*
do
  # 删除 prod/package/ 及其之前的部分
  BINDING_NAME=${path#*/*/}
  # 删除 BINDING_NAME 匹配的第一个斜杠及其之后的部分
  BINDING_ABI=${BINDING_NAME%%/*}
  # 脚本原本是处理 xx.node 文件所有才会有 BINARY_PATH 这个变量扩展语法
  BINARY_PATH=${path%%.*}


  echo "-------------------开始准备变量-------------------"
  echo "文件路径 $path"
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
