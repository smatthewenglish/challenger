use std::error::Error;
use std::env;
use csv;

/// Reads data from a file into a reader and prints all records.
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;
    let mut list: Vec<Account> = Vec::new();
   
    for result in reader.records() {
        let record = result?;

        let client = record[1].parse::<u16>().unwrap();
        let amount = record[3].parse::<f32>().unwrap();
    
		let account00 = Account { client: client,  available: amount};
		if !list.contains(&account00) { 
			list.push(account00);
			continue
		}
		let index = list.iter().position(|&r| r == account00).unwrap();
	    if &record[1] == "deposit" {
			list[index].available += amount
	    }
	    if &record[1] == "withdrawal" {
			list[index].available -= amount
	    }
    }
	println!("client,available,held,total,locked");

    for (_pos, e) in list.iter().enumerate() {
        println!("{},{},0,{},false", e.client, e.available,e.available);
    }
    Ok(())
}

#[derive(Debug, Copy, Clone)]
pub struct Account {
   pub client:    u16,
   pub available: f32
   //pub held:      f64,
   //pub total:     f64,
   //pub locked:    bool
}
impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.client == other.client
    }
}

fn main() {
	let args: Vec<String> = env::args().collect();

    // If an error occurs print error
    if let Err(e) = read_from_file(&args[1]) {
        eprintln!("{}", e);
    }
}
