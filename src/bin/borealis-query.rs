use borealis::client::{generate_auth_token, Aurora, ConfigFile};
use borealis::discovery::discover_aurora;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Searching your network for Nanoleaf Aurora devices...");
    match discover_aurora(Duration::from_secs(30)).await {
        Ok(addr) => {
            println!("Found Nanoleaf Aurora device at {}", addr);
            println!("Beginning the pairing process.");
            println!(
                "To begin, hold the on-off button on the Aurora for \n\
                7 seconds until the LED starts flashing. Then press Enter... "
            );
            let mut buffer = String::new();
            let _ = std::io::stdin().read_line(&mut buffer);
            println!("Generating auth token...");
            let token = generate_auth_token(addr, None).await.unwrap();
            let aurora = Aurora::new(addr, None, token.clone()).unwrap();
            let name = aurora.get_name().await.unwrap();
            let config = ConfigFile::new(addr, token, name);
            config.write().unwrap();
        }
        Err(e) => {
            eprintln!("Encountered error while searching for Aurora: {}", e);
        }
    }
}
