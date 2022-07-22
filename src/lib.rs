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

    let result = {
        if args.number_nonblank {
            nonblank_numbered(contents)
        } else if args.number {
            numbered(contents)
        } else {
            contents
        }
    };

    println!("{}", result);

    Ok(())
}

fn numbered(contents: String) -> String {
    contents
        .lines()
        .enumerate()
        .map(|element| {
            let number = (element.0 + 1).to_string();
            let line = element.1;
            format!("     {}  {}", number, line)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn nonblank_numbered(contents: String) -> String {
    let mut current_line: usize = 1;
    contents
        .lines()
        .map(|line| {
            let result = match line.is_empty() {
                true => line.to_string(),
                false => {
                    let number = current_line.to_string();
                    current_line += 1;
                    format!("     {}  {}", number, line)
                }
            };

            result
        })
        .collect::<Vec<String>>()
        .join("\n")
}
