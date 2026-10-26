#!/bin/bash
# build .deb from this repo root
set -e
cargo build --release
mkdir -p packaging/deb/usr/bin
cp target/release/two-time-shell packaging/deb/usr/bin/two-time
chmod +x packaging/deb/usr/bin/two-time
dpkg-deb --build packaging/deb two-time-shell_1.0.0_amd64.deb
echo "built two-time-shell_1.0.0_amd64.deb"
