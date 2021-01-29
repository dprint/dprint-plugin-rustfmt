#!/bin/bash

# See https://sed.js.org/?gist=733942438b671befe20b6e7b5dfac57f for an explanation of the regex
export CFG_RELEASE=$(cargo --version | sed -rn 's/cargo\s(.*)\s\(.*\)/\1/p')
export CFG_RELEASE_CHANNEL=nightly

cargo build --release --target wasm32-unknown-unknown --verbose
