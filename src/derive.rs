use clap::{ArgAction, Parser};
#[derive(Parser)]
#[command(
    name = "My RPN program",
    author = "nyuta",
    version = "1.0.0",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    //set the level of verbosity
    #[arg(short, long,action= ArgAction::SetTrue)]
    verbose: bool,

    //formulas written in RPN
    formula_file: Option<String>,
}

pub fn run() {
    let cli = Opts::parse();

    match cli.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }
    println!("Is verbosity specified?: {}", cli.verbose);
}
