#!/bin/bash

set -eu
set -o pipefail
readonly DAY="$(ls -d day?? | tail -1)"
cd "${DAY}"
cargo run