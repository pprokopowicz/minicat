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
            nonblank_numbered(&contents)
        } else if args.number {
            numbered_lines(&contents)
        } else {
            contents
        }
    };

    println!("{}", result);

    Ok(())
}

fn numbered_lines(contents: &str) -> String {
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

fn nonblank_numbered(contents: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbered() {
        let contents = "\
Lorem ipsum dolor sit amet
consectetur adipiscing elit
Vestibulum sit amet tellus efficitur";

        let expected = "\
\u{20}    1  Lorem ipsum dolor sit amet
\u{20}    2  consectetur adipiscing elit
\u{20}    3  Vestibulum sit amet tellus efficitur";

        assert_eq!(expected, numbered_lines(contents));
    }

    #[test]
    fn numbered_empty_line() {
        let contents = "\
Lorem ipsum dolor sit amet


consectetur adipiscing elit

Vestibulum sit amet tellus efficitur";

        let expected = "\
\u{20}    1  Lorem ipsum dolor sit amet
\u{20}    2\u{20}\u{20}
\u{20}    3\u{20}\u{20}
\u{20}    4  consectetur adipiscing elit
\u{20}    5\u{20}\u{20}
\u{20}    6  Vestibulum sit amet tellus efficitur";

        assert_eq!(expected, numbered_lines(contents));
    }
}
