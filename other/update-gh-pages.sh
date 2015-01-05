#!/bin/bash

# force errors to quit the script
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
DOC_DIR="$GIT_ROOT/target/doc"
INDEX_PAGE="$DOC_DIR/index.html"

echo "git root: $GIT_ROOT"
echo "doc dir: $DOC_DIR"
echo "index page: $INDEX_PAGE"

cargo clean
cargo doc
echo '<meta http-equiv=refresh content=0;url=portmidi/index.html>' > "$INDEX_PAGE"
ghp-import -n "$DOC_DIR"
echo
echo "Now 'git push -qf origin gh-pages'"

