// MIT - TwoTimeMilestone4 - OWN theme, NOT p10k - lore-coding + OSC fix
use colored::Colorize;
use std::process::Command;

#[derive(Clone)]
pub struct Theme {
    pub name: &'static str,
    pub ok: &'static str,
    pub err: &'static str,
    pub welcome: &'static str,
    pub desc: &'static str,
    pub user_col: (u8,u8,u8),
    pub path_col: (u8,u8,u8),
    pub git_col: (u8,u8,u8),
    pub sym_ok: (u8,u8,u8),
    pub sym_err: (u8,u8,u8),
}

impl Theme {
    pub const SYMBOL: &'static str = "꥟";

    pub fn azuretime() -> Self {
        Self { name: "azuretime", ok: "respawned", err: "despawned", desc: "Family Home", welcome: "\n  ꥟ AZURETIME\n", user_col: (122,165,214), path_col: (45,45,45), git_col: (139,38,53), sym_ok: (122,165,214), sym_err: (139,38,53) }
    }
    pub fn two_time() -> Self {
        Self { name: "two_time", ok: "respawned", err: "failed", desc: "Dark Slate Blue", welcome: "\n  ꥟ TWO TIME\n", user_col: (90,106,138), path_col: (30,42,58), git_col: (20,20,20), sym_ok: (90,106,138), sym_err: (139,38,53) }
    }
    pub fn blue() -> Self {
        Self { name: "azure", ok: "remembered", err: "forgotten", desc: "Azurewrath", welcome: "\n  ꥟ AZURE\n", user_col: (106,76,147), path_col: (10,10,10), git_col: (58,42,74), sym_ok: (106,76,147), sym_err: (200,200,200) }
    }
    pub fn spawn() -> Self {
        Self { name: "spawn", ok: "spawned", err: "no ritual", desc: "Spawn", welcome: "\n  ꥟ SPAWN\n", user_col: (224,224,224), path_col: (10,10,10), git_col: (80,80,80), sym_ok: (224,224,224), sym_err: (139,38,53) }
    }
    pub fn from_name(n: &str) -> Self {
        let low=n.to_lowercase();
        if low.contains("two") { return Self::two_time(); }
        if low=="azure"||low=="blue" { return Self::blue(); }
        if low=="spawn" { return Self::spawn(); }
        Self::azuretime()
    }
    pub fn all() -> Vec<&'static str> { vec!["azuretime","two_time","azure","spawn"] }
    fn git_branch() -> Option<String> {
        let out=Command::new("git").args(["branch","--show-current"]).output().ok()?;
        if out.status.success() {
            let b=String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !b.is_empty() { return Some(b); }
        }
        None
    }
    fn get_user() -> String { std::env::var("USER").or_else(|_| std::env::var("USERNAME")).unwrap_or_else(|_| "user".to_string()) }
    fn get_host() -> String {
        if let Ok(h)=std::env::var("HOSTNAME") { if !h.is_empty() { return h.split('.').next().unwrap_or(&h).to_string(); } }
        if let Ok(out)=Command::new("hostname").output() { let s=String::from_utf8_lossy(&out.stdout).trim().to_string(); if !s.is_empty() { return s; } }
        if let Ok(s)=std::fs::read_to_string("/etc/hostname") { let t=s.trim().to_string(); if !t.is_empty() { return t; } }
        "void".to_string()
    }
    pub fn osc_title(title: &str) -> String { format!("\x1b]0;{}\x07", title) }
    pub fn title(&self, path: &str) -> String {
        let user_host=format!("{}@{}", Self::get_user(), Self::get_host());
        Self::osc_title(&format!("{}:{} - {} - ꥟", user_host, path, self.name))
    }
    pub fn prompt(&self, path: &str, last: i32) -> String {
        let user_host=format!("{}@{}", Self::get_user(), Self::get_host());
        let user_part = format!("{}", user_host).truecolor(self.user_col.0, self.user_col.1, self.user_col.2).bold();
        let path_part = format!("{}", path).truecolor(self.path_col.0, self.path_col.1, self.path_col.2);
        let mut git_part = String::new();
        if let Some(b)=Self::git_branch() {
            git_part = format!(" {{{}}}", b).truecolor(self.git_col.0, self.git_col.1, self.git_col.2).to_string();
        }
        let sym_col=if last==0 { self.sym_ok } else { self.sym_err };
        let sym = format!(" {} ", Self::SYMBOL).truecolor(sym_col.0,sym_col.1,sym_col.2).bold();
        format!("{} {} {}{} {} ", sym, user_part, path_part, git_part, sym)
    }
}
