#!/bin/bash +x

PROJECT_ROOT=/home/jernej/Projects/sicniki_sumniki/sicniki-sumniki-api
BINARY_NAME=filler

alias cargo=/home/jernej/.cargo/bin/cargo

cd $PROJECT_ROOT || exit

cargo +nightly build --release

$PROJECT_ROOT/target/release/$BINARY_NAME