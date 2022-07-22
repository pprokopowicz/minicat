use clap::Parser;
use std::{error::Error, fs};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    number: bool,
    #[clap(value_parser)]
    file: String,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let contents = fs::read_to_string(args.file)?;

    if args.number {
        print_lines(contents)
    } else {
        println!("{}", contents);
    }

    Ok(())
}

fn print_lines(contents: String) {
    let contents_lines: String = contents
        .lines()
        .enumerate()
        .map(|element| {
            let number = (element.0 + 1).to_string();
            let line = element.1;
            format!("     {}  {}", number, line)
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", contents_lines)
}
