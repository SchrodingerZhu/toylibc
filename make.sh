#!/usr/bin/env sh
TARGET_DIR=$PWD/target/raw/debug
cargo build -Zbuild-std=core,alloc --target raw.json
mkdir /tmp/toylibc
cp target/raw/debug/libtoylibc.rlib /tmp/toylibc
cd /tmp/toylibc && llvm-ar -x libtoylibc.rlib && clang -shared ./*.o -Wl -o toylibc.so
cd /tmp/toylibc && cp toylibc.so $TARGET_DIR
rm -rf /tmp/toylibc
