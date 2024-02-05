use clap::Parser;
use kanji_tests;

fn main() {
    let args: kanji_tests::Opts = kanji_tests::Opts::parse();
    let fn_format_output: &dyn Fn(&Vec<kanji_tests::Kanji>, usize) -> String;

    match args.output.as_str() {
        "csv" => fn_format_output = &kanji_tests::format_character_list_as_csv,
        _ => panic!("Output type not recognized"),
    }

    kanji_tests::create_kanji_test(args, &kanji_tests::get_character_list, fn_format_output);
}
