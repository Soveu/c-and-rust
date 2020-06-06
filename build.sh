#!/bin/bash

cd rust/
cargo build 
cd ..
clang++ main.cpp rust/target/debug/librusty.a -std=c++20 -ggdb

