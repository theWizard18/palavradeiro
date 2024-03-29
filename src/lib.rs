use std::{collections::HashMap, env, fs};

use rand::{seq::SliceRandom, Rng};
use serde::Deserialize;

#[derive(Deserialize)]
pub enum Tactic {
    Maybe(Vec<char>),
    One(Vec<char>),
}

#[derive(Deserialize)]
pub struct Config {
    pub phonotactics:       Vec<Tactic>,
    pub phonemes:           HashMap<char, Vec<String>>,
    pub max_syllables:      u8,
    pub word_quantity:      u16,
    pub filters:            Vec<String>,
    pub separate_syllables: bool,
}
impl Config {
    pub fn build(
        phonotactics:       Vec<Tactic>,
        phonemes:           HashMap<char, Vec<String>>,
        max_syllables:      u8,
        word_quantity:      u16,
        filters:            Vec<String>,
        separate_syllables: bool,
    ) -> Self {
        Self {
            phonotactics,
            phonemes,
            max_syllables,
            word_quantity,
            filters,
            separate_syllables,
        }
    }
    fn from_default() -> Self {
        let mut phonemes = HashMap::new();
        phonemes.insert(
            'C',
            vec!["m", "n", "p", "t", "k", "s", "w", "l", "j"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        phonemes.insert(
            'V',
            vec!["a", "e", "i", "o", "u"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        phonemes.insert('N', vec!["m", "n"].iter().map(|s| s.to_string()).collect());
        Self {
            phonemes,
            phonotactics:       vec![Tactic::Maybe(vec!['C']), Tactic::One(vec!['V']), Tactic::Maybe(vec!['N'])],
            filters:            vec![],
            max_syllables:      3,
            separate_syllables: false,
            word_quantity:      100,
        }
    }
}

pub fn run() -> Result<Vec<String>, &'static str> {
    let home = env::var("HOME").map_err(|_| "Couldn't find home directory.")?;

    let paths = vec![
        "./palavradeiro.yaml".into(),
        format!("{}/palavradeiro.yaml", home),
        format!("{}/.config/palavradeiro/palavradeiro.yaml", home),
    ];
    let config = get_config(&paths);
    let mut words = Vec::new();
    let mut none_counter = 0;
    while words.len() < config.word_quantity as usize {
        let word = gen_word(
            &config.phonotactics,
            &config.max_syllables,
            &config.phonemes,
            &config.separate_syllables,
            &config.filters,
        );
        match word {
            None    => none_counter += 1,
            Some(w) => {
                words.push(w);
                none_counter = 0;
            },
        }

        if none_counter >= 100 {
            return Err("The filters are impeding the generation of words,\n\
            change the filters in order to generate them.")
        }
    }
    Ok(words)
}

fn get_config(paths: &Vec<String>) -> Config {
    let mut yaml = "".into();
    for i in paths {
        match fs::read_to_string(i) {
            Ok(f)  => yaml = f,
            Err(_) => continue,
        }
    }
    if yaml.is_empty() {
        println!("Couldn't find `palavradeiro.yaml`, using defaults");
        return Config::from_default();
    }
    match serde_yaml::from_str::<Config>(&yaml) {
        Ok(c)  => c,
        Err(_) => {
            println!("Failed to parse `palavradeiro.yaml`, using defaults");
            Config::from_default()
        },
    }
}
fn gen_word (
    tactics:       &[Tactic],
    max_syllables: &u8,
    phonemes:      &HashMap<char, Vec<String>>,
    sep_syllable:  &bool,
    filters: &Vec<String>,
) -> Option<String> {
    let mut word = String::new();
    let syllable_qtd = rand::thread_rng().gen_range(1..=*max_syllables);

    for _ in 1..=syllable_qtd {
        word.push_str(gen_syllable(tactics, phonemes, sep_syllable).as_str());
    }
    if filters.iter().any(|f| word.contains(f)) {
        return None
    }
    Some(word)
}

fn gen_syllable(
    tactics:      &[Tactic],
    phonemes:     &HashMap<char, Vec<String>>,
    sep_syllable: &bool,
) -> String {
    let chars: Vec<String> = tactics
        .iter()
        .map(|i| process_tactic(i, phonemes))
        .collect();

    let mut syllable = String::new();
    for c in chars {
        syllable.push_str(c.as_str());
    }
    if *sep_syllable {
        syllable.push('-');
    }
    syllable
}

fn process_tactic(group: &Tactic, phonemes: &HashMap<char, Vec<String>>) -> String {
    let key = match group {
        Tactic::One(vc)   => process_one(vc),
        Tactic::Maybe(vc) => process_maybe(vc),
    };
    match key {
        Some(c) => choose_phoneme_of(c, phonemes),
        None    => "".into(),
    }
}

fn process_maybe(vc: &Vec<char>) -> Option<&char> {
    let mut rng = rand::thread_rng();
    match rng.gen_bool(0.5) {
        true  => None,
        false => process_one(vc),
    }
}

fn process_one(vc: &Vec<char>) -> Option<&char> {
    let mut rng = rand::thread_rng();
    vc.choose(&mut rng)
}

fn choose_phoneme_of(group: &char, phonemes: &HashMap<char, Vec<String>>) -> String {
    let mut rng = rand::thread_rng();
    if !phonemes.contains_key(group) {
        return "".into();
    }
    phonemes[group]
        .choose(&mut rng)
        .unwrap_or(&("".into()))
        .to_string()
}
