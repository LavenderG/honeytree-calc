use std::env;
use std::error::Error;
use std::process;

use honeytree_calc::htree;

///
/// Runs the honeytree-calc CLI binary.
///
fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    if args[1] == "-h" || args[1] == "--help" {
        print_help();
        return Ok(());
    }

    let trainer_id =  match args[1].trim().parse() {
        Ok(id) => {
            id
        }
        Err(_) => {
            print_help();
            return Ok(());
        }
    };

    let trainer_secret_id =  match args[2].trim().parse() {
        Ok(id) => {
            id
        }
        Err(_) => {
            print_help();
            return Ok(());
        }
    };

    let trainer_data = htree::result::TrainerData::new(trainer_id, trainer_secret_id);

    println!(
        "Munchlax honey trees for TID {} and SID {} can be found in the following locations:",
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
    println!("Usage: honetree-calc-cli [-h|--help] <trainer id> <trainer secret id>");
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
