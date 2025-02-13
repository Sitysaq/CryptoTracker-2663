```rust
// Importing the required libraries
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// Struct to hold the data
#[derive(Debug)]
struct Data {
    id: u32,
    value: u32,
}

// Function to read data from file
fn read_data_from_file(file: File) -> io::Result<Vec<Data>> {
    let reader = BufReader::new(file);
    let mut data_vec: Vec<Data> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(",").collect();
        let data = Data {
            id: parts[0].parse().unwrap(),
            value: parts[1].parse().unwrap(),
        };
        data_vec.push(data);
    }
    Ok(data_vec)
}

// Function to process data
fn process_data(data_vec: Vec<Data>) -> Vec<Data> {
    let mut processed_data: Vec<Data> = Vec::new();
    for data in data_vec {
        let processed_value = data.value * 2; // just a simple processing for example
        let processed_data_object = Data {
            id: data.id,
            value: processed_value,
        };
        processed_data.push(processed_data_object);
    }
    processed_data
}

// Function to write processed data to file
fn write_data_to_file(data_vec: Vec<Data>, file: File) -> io::Result<()> {
    let mut writer = BufWriter::new(file);

    for data in data_vec {
        writer.write_all(data.id.to_string().as_bytes())?;
        writer.write_all(",".as_bytes())?;
        writer.write_all(data.value.to_string().as_bytes())?;
        writer.write_all("\n".as_bytes())?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // Read data from file
    let file = File::open("data.csv")?;
    let data_vec = read_data_from_file(file)?;

    // Process data
    let processed_data = process_data(data_vec);

    // Write processed data to file
    let file = File::create("processed_data.csv")?;
    write_data_to_file(processed_data, file)
}
```
Цей код читає дані з файла "data.csv", оброблює ці дані (просто помноживши значення на 2) і записує оброблені дані до файла "processed_data.csv".