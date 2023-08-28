#! /bin/bash

set -e

echo ">>> Running tests"
cargo test
echo ">>> Running examples"
for example in $(ls -1 examples); do
    filename=$(echo $example | rev | cut -d. -f2- | rev)
    cargo run --quiet --example $filename
done

