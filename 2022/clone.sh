#!/bin/bash

set -eu
set -o pipefail

main() {
    local day="$(ls -d day?? | tail -1)"
    local daynum="${day#day}"
    daynum="${daynum#0}"
    local newdaynum="$(( daynum + 1 ))"
    local newday="$(printf 'day%02d' "${newdaynum}")"
    (
        set -x
        cp -R template "${newday}/"
        sed "s/{DAY}/${newday}/" < "${newday}/Cargo.toml" > "/tmp/${newday}"
        mv "/tmp/${newday}" "${newday}/Cargo.toml"
        ./fetch.sh || "Fetch failed"
    )
}

main "$@"