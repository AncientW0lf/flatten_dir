mod flattener;

use std::io;
use std::io::Write;

fn main() -> Result<(), io::Error> {
    println!("Enter the full path to the directory you would like to flatten.");

    print!("Directory: ");
    io::stdout().flush().ok();

    let mut directory = String::new();
    io::stdin()
        .read_line(&mut directory)
        .expect("Error reading stdin.");
    directory = String::from(directory.trim());

    std::fs::metadata(&directory)?;

    println!("Flattening directory...");
    flattener::flatten(directory.as_str())?;

    println!("Flattened directory!");

    Ok(())
}
