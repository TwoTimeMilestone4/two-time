%global debug_package %{nil}

Name:           two-time
Version:        1.1.0
Release:        1%{?dist}
Summary:        lore-coding shell with Two Time vibe ꥟
License:        MIT
URL:            https://github.com/TwoTimeMilestone4/two-time
Source0:        https://github.com/TwoTimeMilestone4/two-time/archive/refs/tags/v1.1.0.tar.gz
# GitHub tarball is TwoTimeMilestone4/two-time/archive/v1.0.0.tar.gz -> two-time-1.0.0/
BuildRequires:  cargo rust
Requires:       zsh
Requires:       google-noto-sans-rejang-fonts
Requires:       google-noto-fonts-all

%description
A fragile cultist that holds a terrible secret. Bearing nothing other than a sleek dagger and a manic smile, they're able to earn a second life through risk and sacrifice.
Symbol ꥟ (U+A95F) everywhere. Own theme NOT p10k: ꥟ user@host path {branch} ꥟
Themes: azuretime, two_time, azure, spawn - colors from FORSAKEN Wiki: Dark Slate Blue, Azurewrath deep purple-blue, neon white spawn.
Login shell via chsh -s /usr/bin/two-time - GLORY TO THE SPAWN

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release

%install
install -Dm755 target/release/two-time-shell %{buildroot}%{_bindir}/two-time 2>/dev/null || install -Dm755 target/release/two-time %{buildroot}%{_bindir}/two-time
install -Dm644 LICENSE %{buildroot}%{_datadir}/licenses/%{name}/LICENSE || install -Dm644 LICENSE.md %{buildroot}%{_datadir}/licenses/%{name}/LICENSE || true
install -Dm644 README.md %{buildroot}%{_datadir}/doc/%{name}/README.md

%post
if ! grep -q "/usr/bin/two-time" /etc/shells; then echo "/usr/bin/two-time" >> /etc/shells; fi
echo "  ꥟ two-time installed - chsh -s /usr/bin/two-time"

%files
%{_bindir}/two-time
%{_datadir}/licenses/%{name}/*
%{_datadir}/doc/%{name}/README.md

%changelog
* Wed Nov 4 2026 TwoTimeMilestone4 - 1.1.0-1
- Added:
+ Launch roleplay directly with two-time rp or the aliases roleplay and forsaken; the shell also supports rp.
+ Quick-start lore encounters, select characters and maps, and browse the Forsaken Codex.
+ Roleplay features include freeform speech and emotes, character abilities, health and resource gauges, emotional stances, ambient events, and Two Time’s Second Life revival.
+ Added a shell help command documenting built-ins and roleplay options.
+ README now documents the roleplay roster, maps, features, and CLI usage.

* Mon Oct 26 2026 TwoTimeMilestone4 - 1.0.0-1
- Initial lore-coding release - own theme, no p10k, OSC fix
