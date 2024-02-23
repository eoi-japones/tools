use clap::Parser;
use kanji_tests;

fn main() {
    let args: kanji_tests::Opts = kanji_tests::Opts::parse();
    let fn_format_creation_output: &dyn Fn(&Vec<kanji_tests::Kanji>, usize) -> String;
    let fn_format_solution_output: &dyn Fn(String, &Vec<kanji_tests::Kanji>) -> String;

    match args.output.as_str() {
        "csv" => {
            fn_format_creation_output = &kanji_tests::format_character_list_as_csv;
            fn_format_solution_output = &kanji_tests::solve_kanji_test_as_csv;
        }
        _ => panic!("Output type not recognized"),
    }

    if args.resolve.is_some() {
        kanji_tests::create_kanji_solution(
            args,
            &kanji_tests::get_character_list,
            fn_format_solution_output,
        );
    } else {
        kanji_tests::create_kanji_test(
            args,
            &kanji_tests::get_character_list,
            fn_format_creation_output,
        );
    }
}
