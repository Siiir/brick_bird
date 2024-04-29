#!/bin/bash

# Define source and destination directories
src_dir="./target"
dest_dir="./target/fd_release/execs"

# Create destination directory if it does not exist
mkdir -p "$dest_dir"

# Create hard links in the destination directory with naming based on target triples
ln "$src_dir/x86_64-pc-windows-gnu/fd_release/brick_bird.exe" "$dest_dir/x86_64-pc-windows-gnu.exe"
ln "$src_dir/x86_64-unknown-linux-gnu/fd_release/brick_bird" "$dest_dir/x86_64-unknown-linux-gnu"
ln "$src_dir/aarch64-apple-darwin/fd_release/brick_bird" "$dest_dir/aarch64-apple-darwin"
ln "$src_dir/aarch64-unknown-linux-gnu/fd_release/brick_bird" "$dest_dir/aarch64-unknown-linux-gnu"
ln "$src_dir/aarch64-pc-windows-msvc/fd_release/brick_bird" "$dest_dir/aarch64-pc-windows-msvc"
ln "$src_dir/aarch64-linux-android/fd_release/brick_bird" "$dest_dir/aarch64-linux-android"

