use crate::webhook::DiscordWebHook;
use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use std::io::BufWriter;
use std::{
    collections::HashMap,
    fs::{remove_file, File, OpenOptions},
    path::{Path, PathBuf},
};
mod webhook;

#[derive(Debug, clap::Parser)]
enum Cli {
    Install,
    List,
}

#[derive(Debug, Deserialize)]
struct PluginsFile {
    plugins: HashMap<String, Plugin>,
}

#[derive(Debug, Deserialize)]
struct Plugin {
    version: String,
    url: String,
}

fn list_plugins(log: &mut impl std::io::Write, dir: &Path, plugins: &PluginsFile) -> Result<()> {
    for (name, Plugin { version, .. }) in plugins.plugins.iter() {
        let plugin_path = dir.join(name);
        writeln!(log, "{}v{} @ {}", name, version, plugin_path.display())?;
    }
    Ok(())
}

fn install_plugins(log: &mut impl std::io::Write, dir: &Path, plugins: &PluginsFile) -> Result<()> {
    for (name, Plugin { version, url }) in plugins.plugins.iter() {
        let plugin_path = dir.join(format!("{}.jar", name));
        if plugin_path.exists() {
            writeln!(log, "- {} v{} @ {}", name, version, plugin_path.display())?;
            remove_file(&plugin_path)?;
        }
        writeln!(log, "+ {} v{} @ {}", name, version, plugin_path.display())?;
        let mut res = reqwest::blocking::get(url)?;
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .open(plugin_path)?;
        res.copy_to(&mut f)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let plugin_path = PathBuf::from("plugins.yml");
    assert!(plugin_path.is_file());
    let args = Cli::try_parse()?;
    let mut plugins_file = File::open(&plugin_path)?;
    let plugins_file: PluginsFile = serde_yaml::from_reader(&mut plugins_file)?;
    let dir = plugin_path.parent().unwrap();

    let mut log = BufWriter::new(Vec::new());

    match args {
        Cli::List => list_plugins(&mut log, dir, &plugins_file)?,
        Cli::Install => install_plugins(&mut log, dir, &plugins_file)?,
    }

    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL")?;
    if !webhook_url.is_empty() {
        let webhook = DiscordWebHook::new("minecraft", &webhook_url);
        let content = String::from_utf8(log.into_inner()?)?;
        webhook.post_message(content)?;
    }
    Ok(())
}
