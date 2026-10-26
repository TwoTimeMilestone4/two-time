// MIT License - Copyright (c) 2026 TwoTimeMilestone4
// FORSAKEN Wiki official lore - lore checked
use colored::Colorize;
use std::process::Command;

#[derive(Clone)]
pub struct Theme {
    pub name: &'static str,
    pub ok: &'static str,
    pub err: &'static str,
    pub welcome: &'static str,
    pub desc: &'static str,
}

impl Theme {
    pub const SYMBOL: &'static str = "꥟";
    const SEP: &'static str = "";

    pub fn azuretime() -> Self {
        Self {
            name: "azuretime",
            ok: "respawned",
            err: "despawned",
            desc: "Family Home For The Reborn - formerly Spawn Cult - fake god The Spawn",
            welcome: "
  ꥟ AZURETIME
  ꥟ A fragile cultist that holds a terrible secret.
  ꥟ Two Time + former partner were followers of Family Home For The Reborn
  ꥟ Belief: sacrificing grants second life from The Spawn - GLORY TO THE SPAWN
",
        }
    }
    pub fn two_time() -> Self {
        Self {
            name: "two_time",
            ok: "respawned",
            err: "failed",
            desc: "A fragile cultist that holds a terrible secret. Bearing nothing other than a sleek dagger and a manic smile, they're able to earn a second life through risk and sacrifice.",
            welcome: "
  ꥟ TWO TIME
  ꥟ A fragile cultist that holds a terrible secret.
  ꥟ Bearing nothing other than a sleek dagger and a manic smile,
  ꥟ they're able to earn a second life through risk and sacrifice.
",
        }
    }
    // renamed from azure() to blue() to avoid link-tag injection breaking build
    pub fn blue() -> Self {
        Self {
            name: "azure",
            ok: "remembered",
            err: "forgotten",
            desc: "Former romantic partner, sacrificed for second life",
            welcome: "
  ꥟ AZURE - former romantic partner, sacrificed for Oblation
",
        }
    }
    pub fn spawn() -> Self {
        Self {
            name: "spawn",
            ok: "spawned",
            err: "no ritual",
            desc: "The Spawn - fake god",
            welcome: "
  ꥟ THE SPAWN - Ritual = Spawn Point + Oblation via dagger
",
        }
    }

    pub fn from_name(n: &str) -> Self {
        let low = n.to_lowercase();
        if low.contains("two") { return Self::two_time(); }
        if low == "azure" || low == "blue" { return Self::blue(); }
        if low == "spawn" { return Self::spawn(); }
        Self::azuretime()
    }

    pub fn all() -> Vec<&'static str> {
        vec!["azuretime","two_time","azure","spawn"]
    }

    fn git_branch() -> Option<String> {
        let out = Command::new("git").args(["branch","--show-current"]).output().ok()?;
        if out.status.success() {
            let b = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !b.is_empty() { return Some(b); }
        }
        None
    }

    fn seg(t: &str, bg: (u8,u8,u8), fg: (u8,u8,u8), next: Option<(u8,u8,u8)>) -> String {
        let nb = next.unwrap_or((18,21,26));
        format!("{}{}",
            format!(" {} ", t).on_truecolor(bg.0,bg.1,bg.2).truecolor(fg.0,fg.1,fg.2),
            Self::SEP.truecolor(bg.0,bg.1,bg.2).on_truecolor(nb.0,nb.1,nb.2)
        )
    }

    pub fn prompt(&self, path: &str, last: i32) -> String {
        let dir_bg = (45,45,45);
        let user_bg = (122,165,214);
        let git_bg = (139,38,53);
        let final_bg = (18,21,26);
        let mut p = String::new();
        p += &Self::seg(" azure@void", user_bg, (18,21,26), Some(dir_bg));
        p += &Self::seg(path, dir_bg, (200,205,214), Some(if Self::git_branch().is_some(){git_bg}else{final_bg}));
        if let Some(b) = Self::git_branch() {
            p += &Self::seg(&format!("󰘬 {}", b), git_bg, (255,255,255), Some(final_bg));
        }
        let sym = if last==0 {
            format!(" {} ", Self::SYMBOL).truecolor(122,165,214).bold()
        } else {
            format!(" {} ", Self::SYMBOL).truecolor(139,38,53).bold()
        };
        p + &format!("{} {} ", format!("{}", Self::SEP).on_truecolor(final_bg.0,final_bg.1,final_bg.2).truecolor(final_bg.0,final_bg.1,final_bg.2), sym)
    }
}
