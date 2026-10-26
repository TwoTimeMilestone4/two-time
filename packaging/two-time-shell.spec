Name:           two-time-shell
Version:        1.0.0
Release:        1%{?dist}
Summary:        lore-coding shell with Two Time vibe ꥟
License:        MIT
URL:            https://github.com/TwoTimeMilestone4/two-time-shell
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo rust
Requires:       zsh google-noto-fonts google-noto-sans-fonts

%description
A fragile cultist that holds a terrible secret. Bearing nothing other than a sleek dagger and a manic smile, they're able to earn a second life through risk and sacrifice.
Symbol ꥟ (U+A95F) everywhere. Themes: azuretime, two_time, azure, spawn.
Login shell via chsh -s /usr/bin/two-time
GLORY TO THE SPAWN

%prep
%autosetup

%build
cargo build --release

%install
install -Dm755 target/release/two-time-shell %{buildroot}%{_bindir}/two-time
install -Dm644 LICENSE %{buildroot}%{_datadir}/licenses/%{name}/LICENSE
install -Dm644 README.md %{buildroot}%{_datadir}/doc/%{name}/README.md

%post
if ! grep -q "/usr/bin/two-time" /etc/shells; then
  echo "/usr/bin/two-time" >> /etc/shells
fi
echo "  ꥟ installed — chsh -s /usr/bin/two-time"
echo "  Fedora/RHEL: sudo dnf install google-* -y"
echo "  openSUSE: sudo zypper install google-*"

%files
%{_bindir}/two-time
%{_datadir}/licenses/%{name}/LICENSE
%{_datadir}/doc/%{name}/README.md

%changelog
* Mon Oct 26 2026 TwoTimeMilestone4 - 1.0.0-1
- Initial lore-coding release
