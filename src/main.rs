// MIT - TwoTimeMilestone4 - reads ~/.zshrc + shared config
mod theme;
use theme::Theme;
use colored::Colorize;
use rustyline::DefaultEditor;
use std::{env, process::Command};

fn require_noto() {
    let has = Command::new("sh").arg("-c").arg("fc-list :charset=A95F | grep -qi noto").status().map(|s| s.success()).unwrap_or(false);
    if !has {
        eprintln!("  ꥟ missing glyph U+A95F");
        eprintln!("  Ubuntu: sudo apt install fonts-noto -y");
        eprintln!("  Fedora/RHEL: sudo dnf install google-* -y");
        eprintln!("  openSUSE: sudo zypper install google-*");
        std::process::exit(1);
    }
}

fn get_theme() -> Theme {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 && args[1] == "theme" {
        let t = Theme::from_name(&args[2]);
        if let Some(h) = dirs::home_dir() {
            let d = h.join(".config/two-time");
            let _ = std::fs::create_dir_all(&d);
            let _ = std::fs::write(d.join("theme"), args[2].trim());
            let _ = std::fs::write(d.join("config"), format!("theme={}\n", args[2].trim()));
        }
        return t;
    }
    if let Some(i) = args.iter().position(|a| a == "--theme") {
        if let Some(n) = args.get(i + 1) { return Theme::from_name(n); }
    }
    if let Ok(v) = env::var("TWO_TIME_THEME") { return Theme::from_name(&v); }
    if let Some(home) = dirs::home_dir() {
        let zshrc = home.join(".zshrc");
        if let Ok(content) = std::fs::read_to_string(&zshrc) {
            for line in content.lines() {
                let l = line.trim();
                if l.starts_with("export TWO_TIME_THEME") || l.starts_with("TWO_TIME_THEME=") {
                    if let Some(val) = l.split('=').nth(1) {
                        let clean = val.trim().trim_matches('"').trim_matches(''').trim();
                        if !clean.is_empty() { return Theme::from_name(clean); }
                    }
                }
            }
        }
        if let Ok(c) = std::fs::read_to_string(home.join(".config/two-time/config")) {
            for line in c.lines() {
                if line.trim_start().starts_with("theme=") {
                    let v = line.split('=').nth(1).unwrap_or("azuretime").trim().trim_matches('"');
                    return Theme::from_name(v);
                }
            }
        }
        if let Ok(c) = std::fs::read_to_string(home.join(".config/two-time/theme")) {
            return Theme::from_name(c.trim());
        }
    }
    Theme::azuretime()
}

fn display_path() -> String {
    let cwd = env::current_dir().unwrap_or_default();
    let home = dirs::home_dir().unwrap_or_default();
    let s = cwd.to_string_lossy().to_string();
    let h = home.to_string_lossy().to_string();
    if s.starts_with(&h) { s.replacen(&h, "~", 1) } else { s }
}

fn main() -> rustyline::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--list-themes".to_string()) {
        for t in Theme::all() { println!("{}", t); }
        return Ok(());
    }
    require_noto();
    let th = get_theme();
    println!("{}", th.welcome.truecolor(122, 165, 214).bold());
    let mut rl = DefaultEditor::new()?;
    let mut last = 0;
    loop {
        let prompt = th.prompt(&display_path(), last);
        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() { continue; }
                let _ = rl.add_history_entry(line);
                let p = match shlex::split(line) { Some(v) => v, None => { last = 1; continue; } };
                if p.is_empty() { continue; }
                match p[0].as_str() {
                    "exit" | "quit" => break,
                    "cd" => {
                        let tgt = if p.len() > 1 { p[1].clone() } else { dirs::home_dir().unwrap().to_string_lossy().to_string() };
                        if env::set_current_dir(&tgt).is_err() { eprintln!("  ꥟ cd: no such dir"); last = 1; } else { last = 0; }
                    }
                    "clear" | "respawn" | "spawn" | "c" => {
                        print!("[2J[1;1H");
                        println!("  ꥟ {}", th.ok);
                        last = 0;
                    }
                    _ => {
                        match Command::new(&p[0]).args(&p[1..]).spawn() {
                            Ok(mut ch) => { last = ch.wait().map(|s| s.code().unwrap_or(0)).unwrap_or(1); }
                            Err(_) => { eprintln!("  ꥟ not found: {}", p[0]); last = 127; }
                        }
                    }
                }
            }
            Err(_) => break,
        }
    }
    println!("  ꥟ despawned.");
    Ok(())
}
