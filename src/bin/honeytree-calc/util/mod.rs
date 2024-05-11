use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::str::FromStr;

pub fn ask_input<T: FromStr + Debug>(in_message: &str, err_message: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    loop {
        io::stdout().flush().unwrap();
        print!("{}: ", in_message);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Error: could not read line.");

        if let Ok(input) = input.trim().parse::<T>() {
            return input;
        } else {
            println!("{}\n", err_message);
            input.clear();
        }
    }
}
