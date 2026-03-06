use std::{fs, path::PathBuf};

use regex::Regex;

use crate::constant::{AGENT_MD, BOOT_MD, IDENT_MD, USER_MD};

pub fn remove_think_tags(text: &str) -> String {
    let re = Regex::new(r"(.*\n)*</think>\n?").unwrap();
    let text = re.replace_all(text, "").trim().to_string();

    text
}

pub fn init_workspace(path: String) {
    let boot_path = PathBuf::from(format!("{}/BOOT.md", path.clone()));
    let user_path = PathBuf::from(format!("{}/USER.md", path.clone()));
    let agent_path = PathBuf::from(format!("{}/AGENT.md", path.clone()));
    let ident_path = PathBuf::from(format!("{}/IDENT.md", path.clone()));

    let create_file_if_not_exists = |path: PathBuf, content: &str| {
        if !path.exists() {
            let _ = fs::write(path, content);
        }
    };

    let path = PathBuf::from(&path);

    if !path.exists() {
        let _ = std::fs::create_dir_all(path);
    }

    create_file_if_not_exists(boot_path, BOOT_MD);
    create_file_if_not_exists(user_path, USER_MD);
    create_file_if_not_exists(agent_path, AGENT_MD);
    create_file_if_not_exists(ident_path, IDENT_MD);
}

pub fn agent_workspace(workspace: &String, agent_id: &String) -> String {
    format!("{workspace}/agents/{agent_id}")
}
