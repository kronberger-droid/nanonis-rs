use nanonis_rs::NanonisClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NanonisClient::new("127.0.0.1", 6501)?;

    let names = client.signal_names_get()?;
    for (i, name) in names.iter().enumerate() {
        println!("{}: {}", i, name);
    }

    Ok(())
}
