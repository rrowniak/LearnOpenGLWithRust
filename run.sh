#!/bin/bash

cargo clippy; RUST_BACKTRACE=1 cargo run $1

