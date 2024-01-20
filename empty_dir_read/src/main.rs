use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "/mnt/in-memory-volume/data.txt";

    // Read from the in-memory volume
    let content = fs::read_to_string(file_path).await?;
    
    println!("Data read from in-memory volume: {}", content);
    Ok(())
}
