use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // Define a regular expression to match numbers
    let re = Regex::new(r"\d{1}").unwrap();

    // Get the current directory of the executable
    let current_dir = env::current_dir()?;

    // Construct the path to day_1.txt
    let txt_path = current_dir.join("src").join("day_1.txt");

    let file = File::open(&txt_path)?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    let mut sum: u128 = 0;

    // Read the file line by line
    for line in reader.lines() {
        if let Ok(string) = line.as_deref() {
            let matches: Vec<&str> = re.find_iter(string).map(|m| m.as_str()).collect();

            // Extract the first two matches
            let first_number = matches.get(0).map_or("", |&s| s);
            let second_number = matches.get(matches.len() - 1).map_or("", |&s| s);
            let result: u128 = format!("{}{}", first_number, second_number)
                .parse()
                .unwrap();
            // println!("{}", result);
            sum += result;
        } else {
            // Handle the error case
            eprintln!("Failed to get string from Result");
        }
    }
    println!("{}", sum);
    Ok(())
}
