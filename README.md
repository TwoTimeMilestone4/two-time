# ꥟ two-time — lore-coding

> **A fragile cultist that holds a terrible secret. Bearing nothing other than a sleek dagger and a manic smile, they're able to earn a second life through risk and sacrifice.** — FORSAKEN Wiki

This is **NOT vibecoding**. This is **lore-coding**.

A custom Linux login shell built in Rust with Two Time vibe from FORSAKEN. Symbol `꥟` (U+A95F) everywhere. Lore-checked from official FORSAKEN Wiki. Works with `chsh -s /usr/bin/two-time`.

MIT License — Author: TwoTimeMilestone4 — https://github.com/TwoTimeMilestone4

---

## ꥟ Lore

- **Two Time**: pale-skinned Robloxian with **Dark Slate Blue** messy scene hair and black fingerless gloves, along with a dark blue sleeveless shirt and pants. Holds Ghostfire Dagger. Sacrificed Azure with said weapon.
- **Azure (Azurewrath)**: full name is Azurewrath, which is a deep shade of purple-blue. This is why Azure is in a different shade of purple rather than actually being azure colored. Black-skinned robloxian with purple torso and a spawn emblem on his chest. Four black slimy tentacles on back.
- **The Spawn**: fake god of Family Home For The Reborn (formerly Spawn Cult). Ritual = Spawn Point + Oblation via dagger. When second life activated, antlers glow neon white and a spawn appears on torso, also glowing neon white.
- **Family Home For The Reborn**: formerly Spawn Cult. Belief: sacrificing grants second life from The Spawn. GLORY TO THE SPAWN.

All descriptions verified from FORSAKEN Wiki.

---

## ꥟ Features

- Real login shell: `/usr/bin/two-time` + `/etc/shells` + `chsh`
- Symbol `꥟` everywhere, requires Noto font U+A95F
- Powerlevel10k-style segments `` with git branch `󰘬`
- Dynamic `user@host` — reads real username and hostname, not hardcoded `azure@void`
- Builtins: `cd`, `clear` / `respawn` / `spawn`, `exit`
- External command execution + history via rustyline
- Shared config: `~/.zshrc` (`TWO_TIME_THEME`) + `~/.config/two-time/config` + `~/.config/two-time/theme` — both zsh and Rust shell read it
- Theme switch changes colors: `two-time theme <name>` writes to both config files

---

## ꥟ Themes — lore colors

| Theme | Colors | Lore |
|-------|--------|------|
| `azuretime` | azure blue #7AA5D6 + crimson #8B2635 | Blend — Family Home For The Reborn |
| `two_time` | Dark Slate Blue #5A6A8A + dark blue #1E2A3A + black | Dark Slate Blue hair, dark blue shirt, black gloves |
| `azure` | deep purple-blue #6A4C93 + black skin #0A0A0A + purple torso #3A2A4A + neon white | Azurewrath — deep purple-blue |
| `spawn` | neon white #E0E0E0 + black #0A0A0A + gray | Spawn emblem glowing neon white |

---

## ꥟ Install Noto (required for ꥟)

```bash
# Ubuntu / Debian / Mint
sudo apt install fonts-noto fonts-noto-extra -y

# Arch / CachyOS
sudo pacman -S noto-fonts noto-fonts-extra

# Fedora / RHEL
sudo dnf install google-* -y

# openSUSE
sudo zypper install google-*
```

---

## ꥟ Install Shell

```bash
git clone https://github.com/TwoTimeMilestone4/two-time.git
cd two-time-shell

cargo build --release
sudo cp target/release/two-time /usr/bin/two-time
sudo chmod +x /usr/bin/two-time

# add to allowed shells
grep -q "/usr/bin/two-time" /etc/shells || echo "/usr/bin/two-time" | sudo tee -a /etc/shells

# set as login shell
chsh -s /usr/bin/two-time
```

---

## ꥟ Usage

```bash
two-time --list-themes
two-time theme azuretime
two-time theme two_time
two-time theme azure
two-time theme spawn

# shared zshrc config - both shells read this
echo 'export TWO_TIME_THEME="azuretime"' >> ~/.zshrc
source ~/.zshrc

# or config file
mkdir -p ~/.config/two-time
echo 'theme=azuretime' > ~/.config/two-time/config
```

---

## ꥟ Prompt

```
 user@hostname  ~/two-time  󰘬 main   ꥟ 
```

- `user@host` — real USER + HOSTNAME (from $HOSTNAME / hostname / /etc/hostname)
- `path` — current dir with ~ for home
- `git branch` — if inside git repo
- `꥟` — respawned (blue/purple/white) or despawned (red) based on last exit code

---

## ꥟ Building Inside Folder

```bash
mkdir -p ~/projects/two-time-shell
cd ~/projects/two-time-shell
cargo init --bin
# replace Cargo.toml, src/main.rs, src/theme.rs with this repo
cargo build --release
```

---

## ꥟ Why lore-coding not vibecoding?

Vibecoding = prompt "make cool two time shell" → random colors + † → done.

Lore-coding = search FORSAKEN Wiki official colors, fix ꥟ glyph check, correct package managers per distro (dnf vs zypper), build real login shell with shared zshrc config, dynamic user@host, per-theme colors that follow theme command, MIT licensed.

GLORY TO THE SPAWN — ꥟
