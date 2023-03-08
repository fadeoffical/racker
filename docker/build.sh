#!/usr/bin/env sh

# This script builds the docker image.

## Check if the cargo command is available
if ! command -v cargo > /dev/null 2>&1; then
    echo "(!) Cargo is not installed"
    exit 1
fi

## Check if the docker command is available
if ! command -v docker > /dev/null 2>&1; then
    echo "(!) Docker is not installed"
    exit 1
fi

## Check if the docker daemon is running
if ! docker info > /dev/null 2>&1; then
    echo "(!) Docker daemon is not running"
    exit 1
fi

## Build and copy the binary
cd ".." || exit
cargo build --release
cp target/release/racker docker/racker

## Build the docker image
cd docker || exit
docker build -t "fadeoffical/racker:latest" .

## Be done with it uwu
# (p r o f e s s i o n a l i s m)
echo "(+) Done"
