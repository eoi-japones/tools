use clap::Parser;
use rand::seq::SliceRandom;
use serde::Deserialize;
use serde_yaml;
use std::fs;

pub mod kanji_tests;

#[derive(Parser)]
pub struct Opts {
    pub kanjis: std::path::PathBuf,
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
    #[arg(long = "resolve", short = 'r')]
    pub resolve: Option<std::path::PathBuf>,
}

#[derive(Deserialize, Debug)]
pub struct Kanji {
    pub id: String,
    pub clave: Option<String>,
    pub historia: Option<String>,
    pub componentes: Option<Vec<String>>,
    pub como_componente: Option<Vec<String>>,
}

pub fn create_kanji_test(
    args: Opts,
    fn_get_characters: &dyn Fn(&std::path::PathBuf, bool) -> Vec<Kanji>,
    fn_format_output: &dyn Fn(&Vec<Kanji>, usize) -> String,
) {
    if !args.kanjis.exists() || !args.kanjis.is_dir() {
        panic!("Kanjis folder path invalid")
    }
    let mut kanji_list: Vec<Kanji> = fn_get_characters(&args.kanjis, false);

    let component_path: std::path::PathBuf;
    if args.components.is_some() {
        component_path = args.components.unwrap();
    } else {
        component_path = args.kanjis.join("componentes");
    };

    if !component_path.exists() || !component_path.is_dir() {
        panic!("Component folder path invalid")
    }
    let mut component_list: Vec<Kanji> = fn_get_characters(&component_path, true);

    kanji_list.append(&mut component_list);
    let result: String = fn_format_output(&kanji_list, args.number.try_into().unwrap());

    println!("{}", result);
}

pub fn create_kanji_solution(
    args: Opts,
    fn_get_characters: &dyn Fn(&std::path::PathBuf, bool) -> Vec<Kanji>,
    solve_kanji_test: &dyn Fn(String, &Vec<Kanji>) -> String,
) {
    if !args.kanjis.exists() || !args.kanjis.is_dir() {
        panic!("Kanjis folder path invalid")
    }
    let mut kanji_list: Vec<Kanji> = fn_get_characters(&args.kanjis, false);

    let component_path: std::path::PathBuf;
    if args.components.is_some() {
        component_path = args.components.unwrap();
    } else {
        component_path = args.kanjis.join("componentes");
    };

    if !component_path.exists() || !component_path.is_dir() {
        panic!("Component folder path invalid")
    }
    let mut component_list: Vec<Kanji> = fn_get_characters(&component_path, true);

    kanji_list.append(&mut component_list);

    // if !args.resolve.unwrap().exists() || !args.resolve.unwrap().is_dir() {
    //     panic!("Kanjis folder path invalid")
    // }

    let result: String = solve_kanji_test(
        fs::read_to_string(args.resolve.unwrap()).expect("Unable to read test file"),
        &kanji_list,
    );

    println!("{}", result);
}

pub fn get_character_list(files_path: &std::path::PathBuf, is_component: bool) -> Vec<Kanji> {
    let mut character_list: Vec<Kanji> = Vec::new();
    let character_files = fs::read_dir(files_path).unwrap();

    'files: for file in character_files {
        let file_path = file.as_ref().unwrap().path().display().to_string();

        if file_path.ends_with(".yaml") || file_path.ends_with(".yml") {
            let file_content = fs::File::open(file_path.clone()).unwrap();

            for doc in serde_yaml::Deserializer::from_reader(file_content) {
                let character_data: Result<Kanji, serde_yaml::Error> = Kanji::deserialize(doc);
                match character_data {
                    Ok(mut character_info) => {
                        if is_component && !&character_info.clave.is_none() {
                            let mut actual_clave: String = "".to_owned();
                            actual_clave.push_str(&character_info.clave.unwrap());
                            actual_clave.push_str(&"*");
                            character_info.clave = Some(actual_clave);
                        }
                        character_list.push(character_info)
                    }
                    Err(_error) => {
                        continue 'files;
                    }
                }
            }
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
        if !character.clave.clone().is_none() {
            result.push_str(&character.clave.clone().unwrap());
            result.push_str(&",");
        }
    }

    return result;
}

pub fn solve_kanji_test_as_csv(claves_to_solve: String, kanji_list: &Vec<Kanji>) -> String {
    let mut result: String = "".to_string();

    'clave: for clave in claves_to_solve.split(",") {
        for kanji in kanji_list {
            if !kanji.clave.is_none() && kanji.clave.clone().unwrap() == clave {
                result.push_str(&kanji.id);
                result.push_str(&",");
                continue 'clave;
            }
        }
    }

    return result;
}
