#!/bin/bash
# to create patches use $ diff -u file1.rs file1.new.rs > file1.patch
curl -L https://crates.io/api/v1/crates/rustc-ap-rustc_data_structures/664.0.0/download | tar xzf -
mv rustc-ap-rustc_data_structures-664.0.0 rustc_data_structures
patch rustc_data_structures/Cargo.toml patches/rustc_data_structures_cargo.patch
patch rustc_data_structures/stack.rs patches/rustc_data_structures_stack.patch

curl -L https://crates.io/api/v1/crates/rustfmt-nightly/1.4.18/download | tar xzf -
mv rustfmt-nightly-1.4.18 rustfmt-nightly
patch rustfmt-nightly/src/lib.rs patches/rustfmt-nightly_lib.patch
