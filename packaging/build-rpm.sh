#!/bin/bash
# build .rpm
set -e
# needs rpmbuild
cargo build --release
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
cp packaging/two-time-shell.spec ~/rpmbuild/SPECS/
tar czf ~/rpmbuild/SOURCES/two-time-shell-1.0.0.tar.gz --exclude=target --exclude=.git -C .. two-time-shell
rpmbuild -bb ~/rpmbuild/SPECS/two-time-shell.spec
echo "built RPM in ~/rpmbuild/RPMS/"
