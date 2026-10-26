#!/bin/bash
# build .tar.zst (Arch)
set -e
makepkg -f
echo "built *.tar.zst — requires zsh + noto-fonts"
