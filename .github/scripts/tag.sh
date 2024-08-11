#!/usr/bin/env bash
set -eu

# $(...): 这是命令替换的语法，表示执行括号内的命令并将输出结果作为字符串返回。
# dirname 命令用于从路径中提取出目录部分。
# ${BASH_SOURCE[0]} 是一个特殊变量，用于获取当前正在执行的脚本的路径（可能是相对路径）。
# &> /dev/null: 将标准输出和标准错误输出都重定向到 /dev/null，即丢弃这些输出，不显示在终端上。
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

git pull || true
pnpm i

CARGO_PROJECT_NAME="image-compress-core"
# $1 shell 脚本第一个参数
version="$1"
cargo_version="$(cargo tree -i -p $CARGO_PROJECT_NAME --depth 0 | awk '{print $2}')"

echo "发布 $version with $CARGO_PROJECT_NAME $cargo_version"

# Update swc_core
# (cd ./bindings && ../scripts/update-all-swc-crates.sh || true)

# Update version
(cd ./packages/core && npm version "$version" --no-git-tag-version --allow-same-version || true)
(cd ./packages/minifier && npm version "$version" --no-git-tag-version --allow-same-version || true)
(cd ./bindings && cargo set-version $version -p binding_core_wasm -p binding_minifier_wasm -p binding_typescript_wasm)
(cd ./bindings && cargo set-version --bump patch -p swc_cli)


# Commmit and tag
git add -A
git commit -am "chore: 发布 \`$version\` 项目：\`$CARGO_PROJECT_NAME\` \`$cargo_version\`"
git tag -a -m "$CARGO_PROJECT_NAME $cargo_version" "v$version"

# Update changelog
yarn changelog
git add -A || true
git commit -m 'chore: 更新 changelog' || true

# Publish packages
git push git@github.com:swc-project/swc.git --no-verify
git push git@github.com:swc-project/swc.git --no-verify --tags
