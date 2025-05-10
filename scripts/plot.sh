#!/usr/bin/env bash

set -e

BIN=./target/debug/conc-map-bench
DATA_DIR=results

cargo build

function plot {
    cat "$DATA_DIR/$1.csv" | "$BIN" plot "$DATA_DIR" "$1"
}

# plot ReadHeavy.hashless
# plot Exchange.hashless
# plot RapidGrow.hashless

plot ReadHeavy.std
plot Exchange.std
plot RapidGrow.std

plot ReadHeavy.ahash
plot Exchange.ahash
plot RapidGrow.ahash

plot ReadHeavy.fxhash
plot Exchange.fxhash
plot RapidGrow.fxhash

plot ReadHeavy.foldhash
plot Exchange.foldhash
plot RapidGrow.foldhash
