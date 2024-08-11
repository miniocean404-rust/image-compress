#!/usr/bin/env bash
set -eu


SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

git pull || true
pnpm i

CARGO_PROJECT_NAME="image-compress-core"
version="$1"
cargo_version="$(cargo tree -i -p $CARGO_PROJECT_NAME --depth 0 | awk '{print $2}')"

echo "发布 $version with $CARGO_PROJECT_NAME $cargo_version"

# Update swc_core
(cd ./bindings && ../scripts/update-all-swc-crates.sh || true)


# Update version
(cd ./packages/core && npm version "$version" --no-git-tag-version --allow-same-version || true)
(cd ./packages/minifier && npm version "$version" --no-git-tag-version --allow-same-version || true)
(cd ./bindings && cargo set-version $version -p binding_core_wasm -p binding_minifier_wasm -p binding_typescript_wasm)
(cd ./bindings && cargo set-version --bump patch -p swc_cli)


# Commmit and tag
git add -A
git commit -am "chore: 发布 \`$version\` with \`$CARGO_PROJECT_NAME\` \`$cargo_version\`"
git tag -a -m "$CARGO_PROJECT_NAME $cargo_version" "v$version"


# Update changelog
yarn changelog
git add -A || true
git commit -m 'chore: Update changelog' || true

# Publish packages
git push git@github.com:swc-project/swc.git --no-verify
git push git@github.com:swc-project/swc.git --no-verify --tags
