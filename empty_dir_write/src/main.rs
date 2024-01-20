use std::fs::File;
use std::io::prelude::*;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = "Hello, Kubernetes in-memory volume!";
    let file_path = "/mnt/in-memory-volume/data.txt";

    // Write to the in-memory volume
    fs::write(file_path, data).await?;

    println!("Data written to in-memory volume successfully.");
    Ok(())
}
