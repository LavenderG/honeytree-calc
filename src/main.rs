use std::process;

fn main() {
    if let Err(e) = honeytree_calc::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
