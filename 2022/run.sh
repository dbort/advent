#!/bin/bash

set -eu
set -o pipefail
readonly DAY="$(ls -d day?? | tail -1)"
cd "${DAY}/src"
cargo fmt
RUST_BACKTRACE=1 cargo run
