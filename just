#!/bin/bash
set -e

generate-patch()(
  rm -rf a b

  wasm-pack build --dev --target web
  mv pkg a
  cp -r a b

  touch pkg.patch
  (cd b && patch -p1 < ../pkg.patch)

  read -p "Press ENTER once done making changes to b (a is the original)" enter

  git diff --no-index --no-prefix a b > pkg.patch || true

  rm -rf a b
)

build()(
  if [[ $1 == '--release' ]]; then dev_flag=''; else dev_flag='--dev'; fi
  wasm-pack build $dev_flag --target web
  cd pkg
  patch -p1 < ../pkg.patch
  npm install --omit=dev
)

publish()(
  cd pkg
  npm publish "$@"
)

cd "$(dirname "${BASH_SOURCE[0]}")"; c=$1; shift; "$c" "$@"
