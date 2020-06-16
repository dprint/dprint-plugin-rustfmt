#!/bin/bash
curl -L https://crates.io/api/v1/crates/rustc-ap-rustc_data_structures/664.0.0/download | tar xzf -
mv rustc-ap-rustc_data_structures-664.0.0 rustc_data_structures
patch rustc_data_structures/Cargo.toml patches/rustc_data_structures_cargo.patch
patch rustc_data_structures/stack.rs patches/rustc_data_structures_stack.patch