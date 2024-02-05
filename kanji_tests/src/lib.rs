use clap::Parser;
use rand::seq::SliceRandom;
use serde::Deserialize;
use serde_yaml;
use std::fs;

pub mod kanji_tests;

#[derive(Parser)]
pub struct Opts {
    kanjis: std::path::PathBuf,
    #[arg(long = "components", short = 'c')]
    components: Option<std::path::PathBuf>,
    #[arg(long = "beginner", short = 'b', default_value = "false")]
    beginner: bool,
    #[arg(long = "advanced", short = 'a', default_value = "false")]
    advanced: bool,
    #[arg(long = "number", short = 'n', default_value = "250")]
    number: u32,
    #[arg(long = "output", short = 'o', default_value = "csv")]
    pub output: String,
}

#[derive(Deserialize)]
pub struct Kanji {
    pub id: String,
    pub clave: String,
    pub historia: String,
    pub componentes: Vec<String>,
    pub como_componente: Vec<String>,
}

pub fn create_kanji_test(
    args: Opts,
    fn_get_characters: &dyn Fn(&std::path::PathBuf) -> Vec<Kanji>,
    fn_format_output: &dyn Fn(&Vec<Kanji>, usize) -> String,
) {
    if !args.kanjis.exists() || !args.kanjis.is_dir() {
        panic!("Kanjis folder path invalid")
    }
    let mut kanji_list: Vec<Kanji> = fn_get_characters(&args.kanjis);

    let component_path: std::path::PathBuf;
    if args.components.is_some() {
        component_path = args.components.unwrap();
    } else {
        component_path = args.kanjis.join("componentes");
    };

    if !component_path.exists() || !component_path.is_dir() {
        panic!("Component folder path invalid")
    }
    let mut component_list: Vec<Kanji> = fn_get_characters(&component_path);

    kanji_list.append(&mut component_list);
    let result: String = fn_format_output(&kanji_list, args.number.try_into().unwrap());

    println!("{}", result);
}

pub fn get_character_list(files_path: &std::path::PathBuf) -> Vec<Kanji> {
    let mut character_list: Vec<Kanji> = Vec::new();
    let character_files = fs::read_dir(files_path).unwrap();

    for file in character_files {
        let file_path = file.as_ref().unwrap().path().display().to_string();

        if file_path.ends_with(".yaml") || file_path.ends_with(".yml") {
            let file_content = fs::File::open(file_path).unwrap();
            let character_data: Kanji = serde_yaml::from_reader(file_content).unwrap();
            character_list.push(character_data);
        }
    }

    return character_list;
}

pub fn format_character_list_as_csv(character_list: &Vec<Kanji>, n_of_characters: usize) -> String {
    let selected_characters: Vec<_> = character_list
        .choose_multiple(&mut rand::thread_rng(), n_of_characters)
        .collect();
    let mut result: String = "".to_string();

    for character in selected_characters {
        result.push_str(&character.id);
        result.push_str(&",");
    }

    result.pop();

    return result;
}
