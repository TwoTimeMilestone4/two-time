# ꥟ two-time — lore-coding

> **A fragile cultist that holds a terrible secret. Bearing nothing other than a sleek dagger and a manic smile, they're able to earn a second life through risk and sacrifice.** — FORSAKEN Wiki

This is **NOT vibecoding**. This is **lore-coding**.

A custom Linux login shell built in Rust with Two Time vibe from FORSAKEN. Symbol `꥟` (U+A95F) everywhere. Lore-checked from official FORSAKEN Wiki. Works with `chsh -s /usr/bin/two-time`.

MIT License — Author: TwoTimeMilestone4 — https://github.com/TwoTimeMilestone4/two-time

---

## ꥟ Lore — verified from FORSAKEN Wiki

- **Two Time**: pale-skinned Robloxian with **Dark Slate Blue** messy scene hair and black fingerless gloves, along with a dark blue sleeveless shirt and pants. Holds Ghostfire Dagger. Sacrificed Azure with said weapon.
- **Azure (Azurewrath)**: full name is Azurewrath, which is a deep shade of purple-blue. This is why Azure is in a different shade of purple rather than actually being azure colored. Black-skinned robloxian with purple torso and a spawn emblem on his chest. Four black slimy tentacles on back.
- **The Spawn**: fake god of Family Home For The Reborn (formerly Spawn Cult). Ritual = Spawn Point + Oblation via dagger. When second life activated, antlers glow neon white and a spawn appears on torso, also glowing neon white.
- **Family Home For The Reborn**: formerly Spawn Cult. Belief: sacrificing grants second life from The Spawn. GLORY TO THE SPAWN.

All descriptions verified from FORSAKEN Wiki.

---

## ꥟ Features — final v1.0.0

- Real login shell: `/usr/bin/two-time` + `/etc/shells` + `chsh`
- Symbol `꥟` everywhere, requires Noto font U+A95F only — **no Powerlevel10k, no Nerd Font needed**
- Own theme — NOT p10k — ` ꥟  user@host path {branch}  ꥟ ` — no `` double sep bug, no `||` fallback, no `[]` brackets
- Dynamic `user@host` — reads real USER + HOSTNAME from `$HOSTNAME` / `hostname` / `/etc/hostname`, not hardcoded `azure@void`
- OSC title `\x1b]0;...\x07` printed **separately** from prompt — fixes cursor bug where `sudo` appeared far right
- Builtins: `cd`, `clear` / `respawn` / `spawn`, `exit`
- External command execution + history via rustyline
- Shared config: `~/.zshrc` (`TWO_TIME_THEME`) + `~/.config/two-time/config` — both zsh and Rust shell read it
- Theme switch changes colors: `two-time theme <name>` writes to both config files
- Fixed quote escaping bug: `trim_matches(|c| c == '"' || c == '\'')`

---

## ꥟ Themes — lore colors — theme also changes color

| Theme | Colors | Lore |
| --- | --- | --- |
| `azuretime` | azure blue #7AA5D6 + crimson #8B2635 | Blend — Family Home For The Reborn |
| `two_time` | Dark Slate Blue #5A6A8A + dark blue #1E2A3A + black | Dark Slate Blue hair, dark blue shirt, black gloves |
| `azure` | deep purple-blue #6A4C93 + black skin #0A0A0A + purple torso #3A2A4A + neon white | Azurewrath — deep purple-blue |
| `spawn` | neon white #E0E0E0 + black #0A0A0A + gray | Spawn emblem glowing neon white |

---

## ꥟ Install Noto (required for ꥟)

Only `꥟` needs Noto — no Powerline/Nerd Font needed anymore:

```
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

## ꥟ Install Shell — from GitHub

```
git clone https://github.com/TwoTimeMilestone4/two-time.git
cd two-time

cargo build --release
sudo cp target/release/two-time-shell /usr/bin/two-time
# or target/release/two-time if Cargo.toml name is two-time
sudo cp target/release/two-time /usr/bin/two-time 2>/dev/null || true

sudo chmod +x /usr/bin/two-time
grep -q "/usr/bin/two-time" /etc/shells || echo "/usr/bin/two-time" | sudo tee -a /etc/shells
chsh -s /usr/bin/two-time
```

---


## ꥟ Known Issues & Fixes

### 1. `azure@void` hardcoded
**Issue:** Prompt showed `azure@void` instead of real user.
**Fix:** `get_user()` from `$USER` + `get_host()` from `$HOSTNAME` / `hostname` / `/etc/hostname` — now `twotimems4@AzuretimePC`

### 2. Double separator ``
**Issue:** `... main   ꥟` — two `` in a row.
**Fix:** `seg()` already adds separator to `final_bg`, removed extra separator — now `p + sym` only

### 3. `[]` brackets around user@host
**Issue:** `[twotimems4@AzuretimePC]` looked p10k-like.
**Fix:** Own theme removed `[]` — now ` ꥟  twotimems4@AzuretimePC ~/path {branch}  ꥟ `

### 4. `||` and boxes for `` / `󰘬`
**Issue:** `||` or square boxes when Powerline/Nerd Font missing.
**Fix:** Own theme uses no Powerline — only `꥟` from Noto. Only need `google-*` / `fonts-noto`

### 5. Cursor far right — `sudo` appears at end of line
**Issue:** ` ꥟  ...  ꥟                      sudo` — cursor miscalculated.
**Cause:** OSC title was inside prompt string, `rustyline` counted it as visible length.
**Fix:** `th.title()` prints OSC separately via `print!()` + `flush()` before `readline()`

### 6. Quote escaping
**Issue:** `trim_matches(''')` — broken `'`
**Fix:** `trim_matches(|c| c == '"' || c == '\'')`

---


## ꥟ Usage

```
two-time --list-themes
two-time theme azuretime
two-time theme two_time
two-time theme azure
two-time theme spawn

# shared zshrc config - both shells read this
echo 'export TWO_TIME_THEME="azuretime"' >> ~/.zshrc
source ~/.zshrc
```

---

## ꥟ Prompt — final own theme, not p10k

```
 ꥟  twotimems4@AzuretimePC ~/two-time-shell {main}  ꥟ 
```

- `user@host` — real USER + HOSTNAME, no `azure@void`
- `path` — current dir with ~ for home, e.g. `~/two-time-shell`
- `{branch}` — git branch if inside repo, e.g. `{main}`
- `꥟` — blue/purple/white when ok, red when error
- No ``, no `||`, no `[]`, no double ``

---

## ꥟ Why lore-coding not vibecoding?

Vibecoding = prompt "make cool two time shell" → random colors + † → done, hardcoded `azure@void`, `` double sep, cursor far right with sudo, fails OBS with no internet.

Lore-coding = search FORSAKEN Wiki official colors, fix ꥟ glyph check, correct package managers per distro (dnf vs zypper), fix `azure@void` to dynamic USER@HOST, fix double , fix OSC cursor bug by printing OSC separately, fix quote escaping `'\'`, build real login shell with shared zshrc config, per-theme colors that follow theme command, separate RPM specs for Fedora/openSUSE/Mageia/RHEL with `Requires: zsh`, add `cargo_vendor` service to fix OBS offline builds, MIT licensed.

GLORY TO THE SPAWN — ꥟
