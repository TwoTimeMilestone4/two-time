// MIT - TwoTimeMilestone4 - dynamic user@host + OSC
use colored::Colorize;
use std::process::Command;
#[derive(Clone)]
pub struct Theme {
    pub name: &'static str,
    pub ok: &'static str,
    pub err: &'static str,
    pub welcome: &'static str,
    pub desc: &'static str,
    pub user_bg: (u8,u8,u8),
    pub dir_bg: (u8,u8,u8),
    pub git_bg: (u8,u8,u8),
    pub sym_ok: (u8,u8,u8),
    pub sym_err: (u8,u8,u8),
}
impl Theme {
    pub const SYMBOL: &'static str = "꥟";
    const SEP: &'static str = "";
    const SEP_FALLBACK: &'static str = ">";
    pub fn azuretime() -> Self { Self { name:"azuretime", ok:"respawned", err:"despawned", desc:"Family Home", welcome:"\n  ꥟ AZURETIME\n", user_bg:(122,165,214), dir_bg:(45,45,45), git_bg:(139,38,53), sym_ok:(122,165,214), sym_err:(139,38,53) } }
    pub fn two_time() -> Self { Self { name:"two_time", ok:"respawned", err:"failed", desc:"Dark Slate Blue", welcome:"\n  ꥟ TWO TIME\n", user_bg:(90,106,138), dir_bg:(30,42,58), git_bg:(20,20,20), sym_ok:(90,106,138), sym_err:(139,38,53) } }
    pub fn blue() -> Self { Self { name:"azure", ok:"remembered", err:"forgotten", desc:"Azurewrath", welcome:"\n  ꥟ AZURE\n", user_bg:(106,76,147), dir_bg:(10,10,10), git_bg:(58,42,74), sym_ok:(106,76,147), sym_err:(200,200,200) } }
    pub fn spawn() -> Self { Self { name:"spawn", ok:"spawned", err:"no ritual", desc:"Spawn", welcome:"\n  ꥟ SPAWN\n", user_bg:(224,224,224), dir_bg:(10,10,10), git_bg:(80,80,80), sym_ok:(224,224,224), sym_err:(139,38,53) } }
    pub fn from_name(n: &str) -> Self { let low=n.to_lowercase(); if low.contains("two") { return Self::two_time(); } if low=="azure"||low=="blue" { return Self::blue(); } if low=="spawn" { return Self::spawn(); } Self::azuretime() }
    pub fn all() -> Vec<&'static str> { vec!["azuretime","two_time","azure","spawn"] }
    fn git_branch() -> Option<String> { let out=Command::new("git").args(["branch","--show-current"]).output().ok()?; if out.status.success() { let b=String::from_utf8_lossy(&out.stdout).trim().to_string(); if !b.is_empty() { return Some(b); } } None }
    fn get_user() -> String { std::env::var("USER").or_else(|_| std::env::var("USERNAME")).unwrap_or_else(|_| "user".to_string()) }
    fn get_host() -> String {
        if let Ok(h)=std::env::var("HOSTNAME") { if !h.is_empty() { return h.split('.').next().unwrap_or(&h).to_string(); } }
        if let Ok(out)=Command::new("hostname").output() { let s=String::from_utf8_lossy(&out.stdout).trim().to_string(); if !s.is_empty() { return s; } }
        if let Ok(s)=std::fs::read_to_string("/etc/hostname") { let t=s.trim().to_string(); if !t.is_empty() { return t; } }
        "void".to_string()
    }
    fn seg(t: &str, bg: (u8,u8,u8), fg: (u8,u8,u8), next: Option<(u8,u8,u8)>) -> String {
        let use_fb = Self::use_fallback();
        let sep = if use_fb { Self::SEP_FALLBACK } else { Self::SEP };

        let nb=next.unwrap_or((18,21,26));
        let sep_char = if Self::use_fallback() { Self::SEP_FALLBACK } else { Self::SEP };
        format!("{}{}", format!(" {} ", t).on_truecolor(bg.0,bg.1,bg.2).truecolor(fg.0,fg.1,fg.2), sep_char.truecolor(bg.0,bg.1,bg.2).on_truecolor(nb.0,nb.1,nb.2))
    }
    fn use_fallback() -> bool {
        if let Ok(v) = std::env::var("TWO_TIME_NO_POWERLINE") { if v=="1" || v.to_lowercase()=="true" { return true; } }
        if let Ok(c) = std::fs::read_to_string("/etc/two-time/fallback") { if c.trim()=="1" { return true; } }
        // auto-detect: check if powerline font installed
        if let Ok(out) = std::process::Command::new("sh").arg("-c").arg("fc-list :charset=E0B0 | grep -qi powerline || fc-list | grep -qi Nerd").status() {
            if !out.success() { return true; }
        }
        false
    }
    fn osc_title(title: &str) -> String { format!("]0;{}", title) }
    pub fn prompt(&self, path: &str, last: i32) -> String {
        let final_bg=(18,21,26);
        let user_host=format!("{}@{}", Self::get_user(), Self::get_host());
        let osc=Self::osc_title(&format!("{}:{} - {} - ꥟", user_host, path, self.name));
        let mut p=String::new();
        p+=&Self::seg(&user_host, self.user_bg, (18,21,26), Some(self.dir_bg));
        p+=&Self::seg(path, self.dir_bg, (200,205,214), Some(if Self::git_branch().is_some(){self.git_bg}else{final_bg}));
        if let Some(b)=Self::git_branch() { let git_icon = if Self::use_fallback() { "" } else { "󰘬" }; p+=&Self::seg(&format!("{} {}", git_icon, b), self.git_bg, (255,255,255), Some(final_bg)); }
        let sym_col=if last==0 { self.sym_ok } else { self.sym_err };
        let sym=format!(" {} ", Self::SYMBOL).truecolor(sym_col.0,sym_col.1,sym_col.2).bold();
        let body=p+&format!("{} {} ", format!("{}", Self::SEP).on_truecolor(final_bg.0,final_bg.1,final_bg.2).truecolor(final_bg.0,final_bg.1,final_bg.2), sym);
        format!("{}{}", osc, body)
    }
}
