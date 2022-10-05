
use clap::{Arg, ArgAction, Command};

pub fn run() {
    //builder
    let matches = Command::new("My RPN program")
        .version("1.0.0")
        .author("Nyuta")
        .about("Super awesome sample RPN calculator")
        .arg(
            Arg::new("formula_file")
                .help("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("verbose")
                .help("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    match matches.get_one::<String>("formula_file") {
        Some(file) => println!("File specified: {:?}", file),
        None => println!("No file specified"),
    }
    let verbose = matches.get_flag("verbose");
    println!("Is verbosity specified?: {:?}", verbose)
}
