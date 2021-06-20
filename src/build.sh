#!/bin/sh

env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin pi
env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin bin-repo
env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin server
env RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin source-repo

sudo install -Dm755 target/release/pi /usr/bin/pi
sudo install -Dm755 target/release/bin-repo /usr/bin/bin-repo
sudo install -Dm755 target/release/server /usr/bin/server
sudo install -Dm755 target/release/source-repo /usr/bin/source-repo
sudo install -Dm755 files/xchroot /bin

echo "Finished installing Pi"