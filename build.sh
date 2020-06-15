#!/bin/bash
export CFG_RELEASE=1.45.0-nightly
export CFG_RELEASE_CHANNEL=nightly
cargo +nightly build --release --target wasm32-unknown-unknown --verbose
