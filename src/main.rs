use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use csv::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let mut file = File::open("/Users/wangyuguo/ids721/P3/data.csv")?;
    
    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the CSV data
    let mut reader = Reader::from_reader(contents.as_bytes());
    for result in reader.records() {
        let record = result?;
        // Do something with the record, such as printing it
        println!("{:?}", record);
    }

    Ok(())
}
