#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
# set -o xtrace

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    exit 1
fi

source .env
echo "$DATABASE_URL"

#sqlx database create

sqlx migrate run

#cargo sqlx prepare
#cargo sqlx prepare --check
