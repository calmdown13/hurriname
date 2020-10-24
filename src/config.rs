use serde::{Serialize, Deserialize};

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn get_male_names() -> Vec<String> {
    let mut names_string = include_str!("male_names.in").to_string();
    trim_newline(&mut names_string);
    let names: Vec<&str> = names_string.split('\n').collect();
    names.iter().map(|s| s.to_string()).collect()
}

fn get_female_names() -> Vec<String> {
    let mut names_string = include_str!("female_names.in").to_string();
    trim_newline(&mut names_string);
    let names: Vec<&str> = names_string.split('\n').collect();
    names.iter().map(|s| s.to_string()).collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub male_names: Vec<String>,
    pub female_names: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            male_names: get_male_names(),
            female_names: get_female_names(),
        }
    }
}

pub fn get_config() -> Config {
    let cfg: Config = confy::load("hurriname").unwrap();
    cfg
}
