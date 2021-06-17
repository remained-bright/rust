# 编译

RUSTFLAGS="-C target-feature=+avx,+fma,+sse,+sse2,+sse3,+ssse3,+sse4.1" RUST_BACKTRACE=1 \
cargo +nightly build --release

