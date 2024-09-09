mod parser;

extern crate arc_parser;

use clap::{value_parser, Command, Arg, ArgAction};

fn main() {
    let matches = Command::new("chem_format_converter")
        .version("1.0")
        .author("Me")
        .about("Does awesome things")
        .arg(Arg::new("file")
            .value_parser(value_parser!(String))
            .action(ArgAction::Set)
            .short('f')
            .long("file")
            .required(true)
            .help("target .arc file"))
        .arg(Arg::new("output")
            .value_parser(value_parser!(String))
            .action(ArgAction::Set)
            .short('o')
            .long("output")
            .required(false)
            .help("output file"))
        .get_matches();

    let default_file = "test.xyz".to_string();
    let default_output = "output.arc".to_string();
    let file = matches.get_one("file").unwrap_or(&default_file);
    let output = matches.get_one("output").unwrap_or(&default_output);
    let mut blocks = Vec::new();
    let structure_block = parser::xyz_parser::read_file(file.clone()).unwrap();
    blocks.push(structure_block);
    arc_parser::parser::file_parser::write_to_file(blocks, output.to_string()).unwrap();
    println!("converted {} to {}", file, output);
}
