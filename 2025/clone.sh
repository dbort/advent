#!/bin/bash

set -eu
set -o pipefail

readonly WORKSPACE_TOML='../Cargo.toml'


main() {
    local year="$(basename $PWD)"
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

        awk -v new_member="${year}/${newday}" '
        /<INSERT_MEMBER/ {
            print "  \"" new_member "\",";
            print;
            next;
        }
        { print }
        ' "${WORKSPACE_TOML}" > /tmp/$$-clone && mv /tmp/$$-clone "${WORKSPACE_TOML}"

        ./fetch.sh || echo "Fetch failed"
    )
}

main "$@"