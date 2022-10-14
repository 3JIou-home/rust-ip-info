#!/usr/bin/env sh
set -x

docker run --rm \
    --interactive \
    --workdir /rust-ip-info \
    --volume $(pwd):/rust-ip-info \
    --env HTTP_PROXY=${_HTTP_PROXY:=} \
    --env USERID=$(id -u) \
    --env GROUPID=$(id -g) \
    rust:1.64 \
    cargo build --release
