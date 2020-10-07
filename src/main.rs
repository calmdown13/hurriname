use structopt::StructOpt;
use rand::Rng;
use strfmt::strfmt;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::prelude::*;
mod config;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "")]
    letter: String,
    #[structopt(short, long,  default_value = "any")]
    gender: String,
    #[structopt(short, long,  default_value = "{name}")]
    format: String,
    #[structopt(short, long)]
    capitalize: bool,
}

fn append(vec_1: Vec<String>, vec_2: Vec<String>) -> Vec<String> {
    let mut vec_combined = vec_1.clone();
    vec_combined.extend(vec_2.iter().cloned());
    vec_combined
}

fn filter_by_letter(names: Vec<String>, letter: String) -> Vec<String> {
    let mut names_filt: Vec<String> = vec![];
    for name in names {
        if name[0..1].to_lowercase() == letter.to_lowercase() {
            names_filt.push(name);
        }
    }
    names_filt
}

fn get_names(cfg: config::Config, gender: String, letter: String) -> Vec<String> {
    let mut names = match &*gender {
        "male" => cfg.male_names,
        "female" => cfg.female_names,
        _ => append(cfg.male_names, cfg.female_names),
    };
    if !letter.is_empty() {
        names = filter_by_letter(names, letter);
    }
    names
}

fn get_name(names: &Vec<String>, capitalize: bool) -> String {
    let idx = rand::thread_rng().gen_range(0, names.len());
    let name = names[idx].clone();
    match capitalize {
        true => name,
        false => name.to_lowercase(),
    }
}

fn format_name(name: String, format: String) -> String {
    let mut vars = HashMap::new();
    vars.insert("name".to_string(), name);
    vars.insert("uuid".to_string(), Uuid::new_v4().to_string());
    let now = Local::now();
    vars.insert("date".to_string(), format!("{}{}{}", now.year(), now.month(), now.day()));
    vars.insert("datetime".to_string(), format!("{}{}{}{}{}{}", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second()));
    let formatted = strfmt(&format, &vars).expect("format incorrect");
    formatted
}

fn main() {
    let args = Cli::from_args();
    let cfg = config::get_config();
    let names = get_names(cfg, args.gender, args.letter);
    let name = get_name(&names, args.capitalize);
    let formatted_name = format_name(name, args.format);
    println!("{}", formatted_name);
}
 