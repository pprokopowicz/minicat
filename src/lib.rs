use clap::Parser;
use std::{error::Error, fs};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'b', long)]
    number_nonblank: bool,
    #[clap(short, long)]
    number: bool,
    #[clap(value_parser)]
    file: String,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let contents = fs::read_to_string(args.file)?;

    if args.number_nonblank {
        print_nonblank_number(contents)
    } else if args.number {
        print_number(contents)
    } else {
        println!("{}", contents);
    }

    Ok(())
}

fn print_number(contents: String) {
    let numbered_contents: String = contents
        .lines()
        .enumerate()
        .map(|element| {
            let number = (element.0 + 1).to_string();
            let line = element.1;
            format!("     {}  {}", number, line)
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", numbered_contents)
}

fn print_nonblank_number(contents: String) {
    let mut current_line: usize = 1;
    let numbered_contents: String = contents
        .lines()
        .map(|line| {
            let result = match line.is_empty() {
                true => line.to_string(),
                false => {
                    let number = current_line.to_string();
                    current_line += 1;
                    format!("     {}  {}", number, line)
                },
            };

            result
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", numbered_contents)
}
