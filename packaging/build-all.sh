#!/bin/bash
set -e
echo "=== .deb ==="
bash packaging/build-deb.sh
echo "=== .tar.zst (Arch) ==="
# makepkg needs to be in clean dir, skip if not Arch
if command -v makepkg &> /dev/null; then bash packaging/build-arch.sh; else echo "skip makepkg — not Arch"; fi
echo "=== .rpm ==="
if command -v rpmbuild &> /dev/null; then bash packaging/build-rpm.sh; else echo "skip rpmbuild — install rpm-build"; fi
