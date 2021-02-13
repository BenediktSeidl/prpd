#!/bin/bash

OPENWRT_SDK=`pwd`/dev/openwrt-sdk-19.07.6-ath79-generic_gcc-7.5.0_musl.Linux-x86_64/staging_dir/toolchain-mips_24kc_gcc-7.5.0_musl

export CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_LINKER=$OPENWRT_SDK/bin/mips-openwrt-linux-gcc
export TARGET_CC=$OPENWRT_SDK/bin/mips-openwrt-linux-gcc
export PKG_CONFIG_ALLOW_CROSS=1
#export OPENSSL_DIR=/tmp/d/openwrt-sdk-19.07.6-ath79-generic_gcc-7.5.0_musl.Linux-x86_64/staging_dir/toolchain-mips_24kc_gcc-7.5.0_musl/
cargo build --target mips-unknown-linux-musl --release
BIN=target/mips-unknown-linux-musl/release/prpd
ls -lah $BIN
$OPENWRT_SDK/bin/mips-openwrt-linux-strip $BIN
upx target/mips-unknown-linux-musl/release/prpd
ls -lah $BIN
