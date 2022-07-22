use std::process;

fn main() {
    if let Err(e) = minicat::run() {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
