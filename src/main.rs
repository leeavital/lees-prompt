use colored::Colorize;
use serde::Deserialize;
use serde_yaml;
use colored::control;
use std::format;
use std::fs;
use std::process::Command;
use std::str;
use dirs;

fn main() {
    // env::set_var("CLICOLOR_FORCE", "yes");
    control::set_override(true);


    let u = get_username();
    let b = get_current_branch()
        .map(|x| format!("on {} ", x.yellow()))
        .unwrap_or("".to_string());
    let p = format!("({})", get_pwd());

    let kc = get_kube_context().unwrap_or("".to_string());

    print!("{} {} {}\n{}$ ", u.green(), p.bold(), b, kc);
}

fn get_kube_context() -> Option<String> {
    let mut kpath = dirs::home_dir()?;
    kpath.push(".kube");
    kpath.push("config");
    let read = fs::read_to_string(kpath).ok()?;
    let kube_config: KubeConfig = serde_yaml::from_str(&read).ok()?;

    for ctx in &kube_config.contexts {
        if ctx.name == kube_config.current_context {
            let s = "".to_string();
            let ns = ctx.context.namespace.as_ref().unwrap_or(&s);

            let s = format!("{}/{} ", ctx.name, ns);
            if s.contains("prod") {
                return Some(s.red().to_string());
            } else {
                return Some(s.yellow().to_string());
            }
        }
    }

    return None;
}

fn get_username() -> String {
    let v = Command::new("whoami").output();

    match v {
        Ok(r) => str::from_utf8(&r.stdout).unwrap().trim().to_string(),
        Err(_e) => "".to_string(),
    }
}

fn get_current_branch() -> Option<String> {
    let v = Command::new("sh")
        .arg("-c")
        .arg("git branch --quiet --points-at  HEAD  --no-color")
        .output()
        .ok()?;

    let branches = str::from_utf8(&v.stdout).unwrap();
    let lines = branches.rsplit('\n');
    for l in lines {
        if l.starts_with("* ") {
            return Some(l.trim_start_matches("* ").to_string());
        }
    }
    return None
}

fn get_pwd() -> String {
    Command::new("pwd")
        .output()
        .map(|x| str::from_utf8(&x.stdout).unwrap().trim().to_string())
        .unwrap_or("".to_string())
}

#[derive(Debug, Deserialize)]
struct KubeConfig {
    contexts: Vec<KubeContext>,
    #[serde(rename = "current-context")]
    current_context: String,
}

#[derive(Debug, Deserialize)]
struct KubeContext {
    name: String,
    context: KubeContextSpec,
}

#[derive(Debug, Deserialize)]
struct KubeContextSpec {
    namespace: Option<String>,
}
