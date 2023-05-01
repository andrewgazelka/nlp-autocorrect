use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}")
    }
}

fn run() -> anyhow::Result<()> {
    let words = read_words()?;

    let has_andrew = words.contains("andrew");
    let has_sakldjhasdljk = words.contains("sakldjhasdljk");
    println!("Has andrew: {}", has_andrew);
    println!("Has sakldjhasdljk: {}", has_sakldjhasdljk);
    Ok(())
}

fn read_words() -> anyhow::Result<HashSet<String>> {
    let mut words = HashSet::new();
    let file = File::open("words.txt").expect("file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        words.insert(line?);
    }
    Ok(words)
}
