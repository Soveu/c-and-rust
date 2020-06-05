#!/bin/bash

cd rust/
cargo build --release
cd ..
clang++ main.cpp rust/target/release/librusty.a -std=c++20 -O3 -flto

