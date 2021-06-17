#!/usr/bin/env bash

DIR=$(cd "$(dirname "$0")"; pwd)
set -ex
cd $DIR

export NDK=$DIR/../android-ndk-r21e
export ANDROID_NDK_HOME=$NDK
export TOOLCHAIN=$NDK/toolchains/llvm/prebuilt/linux-x86_64
export TARGET=aarch64-linux-android
export API=30

export PATH=$TOOLCHAIN/bin:$PATH
export AR=$TOOLCHAIN/bin/$TARGET-ar
export AS=$TOOLCHAIN/bin/$TARGET-as
export CC=$TOOLCHAIN/bin/$TARGET$API-clang
export CXX=$TOOLCHAIN/bin/$TARGET$API-clang++
export LD=$TOOLCHAIN/bin/$TARGET-ld
export RANLIB=$TOOLCHAIN/bin/$TARGET-ranlib
export STRIP=$TOOLCHAIN/bin/$TARGET-strip

echo 'AR = '$AR
echo 'AS = '$AS
echo 'CC = '$CC
echo 'CXX = '$CXX
echo 'LD = '$LD
echo 'RANLIB = '$RANLIB
echo 'STRIP = '$STRIP


#V8_FROM_SOURCE=1
#GN_ARGS='target_os="android" target_cpu="arm"'
#RUSTFLAGS="-C target-feature=+aes" \
RUST_BACKTRACE=1 \
cargo build --release --target aarch64-linux-android


