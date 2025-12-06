#!/bin/bash
set -eu
set -o pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)

DEFAULT_ARGS="sample-input.txt"
NEW_ARGS=${@:-$DEFAULT_ARGS}
set -- $NEW_ARGS

cd "${SCRIPT_DIR}"
readonly DAY="$(ls -d day?? | tail -1)"
cd "${DAY}"
cargo fmt
cd "src"
RUST_BACKTRACE=1 cargo run -- "$@"
