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
cargo_project_version="$(cargo tree -i -p $CARGO_PROJECT_NAME --depth 0 | awk '{print $2}')"

echo "发布 $version with $CARGO_PROJECT_NAME $cargo_project_version"

# Update swc_core
# (cd ./bindings && ../scripts/update-all-swc-crates.sh || true)

# Update version
# npm version: 这是 npm 的一个命令，用于修改项目的 package.json 文件中的版本号。
# --no-git-tag-version: 这个标志阻止 npm 在更新版本后自动创建一个 Git 标签。如果你不想将版本更新与 Git 提交关联，可以使用这个选项
# --allow-same-version: 这个标志允许将版本号设置为与当前版本相同。在某些情况下，可能需要在不更改版本号的情况下重新发布包，此选项有助于实现这一点。
(cd ./packages/core && npm version "$version" --no-git-tag-version --allow-same-version || true)
(cd ./packages/minifier && npm version "$version" --no-git-tag-version --allow-same-version || true)
# cargo-edit 命令
# cargo set-version 设置版本号
# --bump patch: 自动增加版本号的 patch 部分，版本标签（major, minor, patch）例如，如果当前版本是 1.0.0，执行此命令后，版本会更新为 1.0.1
# -p 指定 workspace 中的包，可以指定多个
(cd ./bindings && cargo set-version $version -p binding_compress_wasm)
# (cd ./bindings && cargo set-version --bump patch -p swc_cli)

# Commmit and tag
git add -A
git commit -am "chore: 发布 \`$version\` 项目：\`$CARGO_PROJECT_NAME\` \`$cargo_project_version\`"
git tag -a -m "$CARGO_PROJECT_NAME $cargo_project_version" "v$version"

# Update changelog
pnpm changelog
git add -A || true
git commit -m 'chore: 更新 changelog' || true

# Publish packages
git push git@github.com:miniocean404-rust/image-compress.git --no-verify
git push git@github.com:miniocean404-rust/image-compress.git --no-verify --tags
