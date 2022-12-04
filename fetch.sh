#!/bin/bash

set -eu
set -o pipefail

main() {
    local day="$(ls -d day?? | tail -1)"
    local daynum="${day#day}"
    daynum="${daynum#0}"
    local input="${day}/src/input.txt"
    echo "> ${input}"
    bzn -y 2022 -d "${daynum}" > "${input}"
}

main "$@"