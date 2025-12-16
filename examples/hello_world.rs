use nanonis_rs::NanonisClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NanonisClient::new("127.0.0.1", 6501)?;

    if let Some(names) = client.signal_names_get()? {
        names.iter().enumerate().map(|(i, n)| println!("{i}:{n}"));
    }

    Ok(())
}
