use borealis::client::generate_auth_token;
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    println!("Beginning the pairing process.");
    println!(
        "To begin, hold the on-off button on the Aurora for \n\
              7 seconds until the LED starts flashing. Then press Enter... "
    );
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    println!("Generating auth token...");
    let token = generate_auth_token(Ipv4Addr::new(192, 168, 1, 19), None)
        .await
        .unwrap();
    println!("New token {}", token);
}
