mod config;

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

use config::Configuration;

const MASTER_ENV_FILENAME: &str = "../.masterenv";

fn main() -> io::Result<()> {
    println!("Start synchronization...");

    let masterenv_var_map = load_masterenv_file()?;
    let project_dir_path = get_project_dir_path()?;

    check_dir_recursive(&project_dir_path, &masterenv_var_map)?;

    println!("Synchronization complete.");
    Ok(())
}

fn get_masterenv_path() -> io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let masterenv_path = current_dir.join(MASTER_ENV_FILENAME);

    if !masterenv_path.exists() {
        eprintln!("Error: {} not found in current directory.", MASTER_ENV_FILENAME);
        std::process::exit(1);
    }
    Ok(masterenv_path)
}

fn get_project_dir_path() -> io::Result<PathBuf> {
    let current_dir = env::current_dir()?;

    let parent_dir = current_dir
        .parent().and_then(|p| p.parent())
        .expect("Error: wrong position, masterenv is positioned on root.");

    Ok(parent_dir.to_path_buf())
}

fn load_masterenv_file() -> io::Result<HashMap<String, String>> {
    let masterenv_path = get_masterenv_path()?;
    let masterenv_file = BufReader::new(File::open(&masterenv_path)?);

    let mut masterenv_var_map = HashMap::new();

    for line in masterenv_file.lines() {
        let line = line?;
        if let Some((var_name, var_value)) = split_var_name_value(&line) {
            masterenv_var_map.insert(var_name, var_value);
        }
    }

    Ok(masterenv_var_map)
}

fn split_var_name_value(line: &str) -> Option<(String, String)> {
    let line_characters = line.trim();
    if line_characters.is_empty() || line_characters.starts_with('#') {
        return None;
    }

    if let Some(line_splitter) = line_characters.find('=') {
        let var_name = line_characters[..line_splitter].trim().to_string();
        let var_value = line_characters[line_splitter + 1..].trim().to_string();
        return Some((var_name, var_value));
    }
    None
}

fn check_dir_recursive(dir_path: &Path, masterenv_var_map: &HashMap<String, String>) -> io::Result<()> {
    if !dir_path.is_dir() {
        return Ok(());
    }

    for child in fs::read_dir(dir_path)? {
        let child_path = child?.path();
        let child_name = get_file_name(&child_path);

        if child_path.is_dir() && !Configuration::is_ignored(&child_name) {
            check_dir_recursive(&child_path, masterenv_var_map)?;
        }

        if !child_path.is_dir() && Configuration::is_config_file(&child_path) {
            check_file(&child_path, masterenv_var_map)?;
        }
    }
    Ok(())
}

fn get_file_name(path: &Path) -> &str {
    path.file_name().and_then(|n| n.to_str()).unwrap_or("")
}

fn check_file(file_path: &Path, masterenv_var_map: &HashMap<String, String>) -> io::Result<()> {
    let file = fs::read_to_string(file_path)?;

    let mut new_lines = Vec::new();
    let mut has_wrong_line = false;

    for line in file.lines() {
        let line_expected = get_line_expected(line, masterenv_var_map);

        if line_expected != line {
            has_wrong_line = true;
        }

        new_lines.push(line_expected);
    }

    if has_wrong_line {
        let new_file = new_lines.join("\n");
        fs::write(file_path, new_file)?;

        println!(" - {:?} has been successfully updated !", file_path);
    }

    Ok(())
}

fn get_line_expected(line: &str, masterenv_var_map: &HashMap<String, String>) -> String {
    let Some((var_name, _)) = split_var_name_value(line) else {
        return line.to_string();
    };

    let Some(master_value) = masterenv_var_map.get(&var_name) else {
        return line.to_string();
    };

    format!("{}={}", var_name, master_value)
}