use std::error::Error;
use std::process;

use honeytree_calc::htree;

mod util;

///
/// Runs the honeytree-calc interactive binary.
/// It will ask for two numbers, TID and SID, and will print the Munchlax honey trees for those two IDs.
///
fn run() -> Result<(), Box<dyn Error>> {
    print_help();

    let trainer_id: u16 = util::ask_input(
        "Input your trainer ID (number between 0 and 65535)",
        "Invalid trainer ID. Please input a number between 0 and 65535.",
    );

    let trainer_secret_id: u16 = util::ask_input(
        "Input your trainer secret ID (number between 0 and 65535)",
        "Invalid trainer secret ID. Please input a number between 0 and 65535.",
    );

    let trainer_data = htree::result::TrainerData::new(trainer_id, trainer_secret_id);

    println!(
        "\nMunchlax honey trees for TID {} and SID {} can be found in the following locations:",
        trainer_id, trainer_secret_id
    );
    trainer_data
        .get_honey_trees()
        .into_iter()
        .for_each(|tree| println!("\t{}", tree.location));

    Ok(())
}

fn print_help() {
    println!("DPPt Honey Tree Calculator");
    println!("This program calculates the Munchlax honey trees based on the trainer's ID an SID.");
    println!();
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
