use chrono::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use strfmt::strfmt;
use structopt::StructOpt;
use uuid::Uuid;
mod config;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "")]
    letter: String,
    #[structopt(short, long, default_value = "")]
    previous_letter: String,
    #[structopt(short, long, default_value = "any")]
    sex: String,
    #[structopt(short, long, default_value = "{name}")]
    format: String,
    #[structopt(short, long)]
    capitalize: bool,
}

fn validate_args(args: &Cli) -> Result<(), &'static str> {
    if !["any".to_string(), "female".to_string(), "male".to_string()].contains(&args.sex) {
        return Err("Sex must be one of [any, female, male].");
    }
    if !args.letter.is_empty() && !args.previous_letter.is_empty() {
        return Err("You cannot pass both letter and previous-letter at the same time.");
    }
    if !args.letter.is_empty() && args.letter.len() != 1 {
        return Err("The letter option only accepts one character.");
    }
    if !args.previous_letter.is_empty() && args.previous_letter.len() != 1 {
        return Err("The previous-letter option only accepts one character.");
    }
    if !args.letter.is_empty() && !args.letter.chars().next().unwrap().is_ascii_alphabetic() {
        return Err("The letter option only accepts alphabetic characters.");
    }
    if !args.previous_letter.is_empty()
        && !args
            .previous_letter
            .chars()
            .next()
            .unwrap()
            .is_ascii_alphabetic()
    {
        return Err("The previous-letter option only accepts alphabetic characters.");
    }
    Ok(())
}

fn get_next_letter(letter_string: &String) -> String {
    let letter_char = letter_string.chars().next().unwrap();
    assert_eq!(1, letter_string.len());
    assert!(letter_char.is_ascii_alphabetic());
    let offset;
    if letter_char.is_ascii_lowercase() {
        offset = 'a' as u8;
    } else {
        offset = 'A' as u8;
    }
    let letter_int = letter_char as u8 - offset;
    ((((letter_int + 1) % 26) + offset) as char).to_string()
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

fn get_names(
    cfg: config::Config,
    sex: String,
    letter: String,
    previous_letter: String,
) -> Vec<String> {
    let mut names = match &*sex {
        "male" => cfg.male_names,
        "female" => cfg.female_names,
        _ => append(cfg.male_names, cfg.female_names),
    };
    if !letter.is_empty() {
        names = filter_by_letter(names, letter);
    } else if !previous_letter.is_empty() {
        names = filter_by_letter(names, get_next_letter(&previous_letter));
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
    vars.insert(
        "date".to_string(),
        format!("{}{}{}", now.year(), now.month(), now.day()),
    );
    vars.insert(
        "datetime".to_string(),
        format!(
            "{}{}{}{}{}{}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        ),
    );
    let formatted = strfmt(&format, &vars).expect("format incorrect");
    formatted
}

fn main() -> Result<(), &'static str> {
    let args = Cli::from_args();
    if let Err(e) = validate_args(&args) {
        return Err(e);
    }
    let cfg = config::get_config();
    let names = get_names(cfg, args.sex, args.letter, args.previous_letter);
    let name = get_name(&names, args.capitalize);
    let formatted_name = format_name(name, args.format);
    println!("{}", formatted_name);
    Ok(())
}
