use clap::Parser;

#[derive(Parser)]
struct Cli {
    components: std::path::PathBuf,
    // #[arg(required = false)]
    // beginner: bool,
    // #[arg(required = false)]
    // advanced: bool,
    number: u32,
    output: String,
}

fn main() {
    let args = Cli::parse();
    println!(
        "{:?}, {:?}, {:?}",
        args.components, args.number, args.output
    );
}
