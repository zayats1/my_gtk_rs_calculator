cargo build
PKG_CONFIG_SYSROOT_DIR=/usr/x86_64-w64-mingw32/ cargo build --target x86_64-pc-windows-gnu

cargo build --release
PKG_CONFIG_SYSROOT_DIR=/usr/x86_64-w64-mingw32/ cargo build --release --target x86_64-pc-windows-gnu
