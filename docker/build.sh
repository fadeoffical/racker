#!/usr/bin/env bash

# This script builds the docker image.

## Check if the cargo command is available
if ! command -v cargo > /dev/null 2>&1; then
    echo "[racker-docker]: Build::check: Cargo is not installed!"
    exit 1
fi

## Check if the docker command is available
if ! command -v docker > /dev/null 2>&1; then
    echo "[racker-docker]: Build::check: Docker is not installed!"
    exit 1
fi

## Check if the docker daemon is running
if ! docker info > /dev/null 2>&1; then
    echo "[racker-docker]: Build::check: Docker daemon is not running!"
    exit 1
fi

## Build and copy the binary
echo -n "[racker-docker]: Build::cargo_build... "
cd ".." || exit
cargo build --release > /dev/null 2>&1
cp target/release/racker docker/racker > /dev/null 2>&1
echo "OK"

## Build the docker image
echo -n "[racker-docker]: Build::docker_build... "
cd docker || exit
docker build -t "fadeoffical/racker:latest" . > /dev/null 2>&1
echo "OK"

## Clean up
echo -n "[racker-docker]: Build::clean... "
rm racker
echo "OK"

## Done
echo "[racker-docker]: Build::done: Build complete!"
