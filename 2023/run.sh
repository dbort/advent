#!/bin/bash
set -eu
set -o pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)

cd "${SCRIPT_DIR}"
readonly DAY="$(ls -d day?? | tail -1)"
cd "${DAY}"
cargo fmt
cd "src"
RUST_BACKTRACE=1 cargo run
