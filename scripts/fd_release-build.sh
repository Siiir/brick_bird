#!/bin/bash

# Build for Windows (64-bit)
cross build --target x86_64-pc-windows-gnu --profile=fd_release

# Build for Linux (64-bit)
cross build --target x86_64-unknown-linux-gnu --profile=fd_release

# Build for macOS (ARM 64-bit, M1/M2 chips)
cross build --target aarch64-apple-darwin --profile=fd_release

# Build for Linux (ARM 64-bit, useful for servers or ARM-based PCs)
cross build --target aarch64-unknown-linux-gnu --profile=fd_release

# Build for Windows (ARM 64-bit, for ARM-based Windows devices)
cross build --target aarch64-pc-windows-msvc --profile=fd_release

# Build for Android (ARM 64-bit)
cross build --target aarch64-linux-android --profile=fd_release

