use borealis;

#[tokio::main]
async fn main() {
    println!("Searching for Nanoleaf Aurora...");
    match borealis::discover_aurora(std::time::Duration::from_secs(30)).await {
        Ok(addr) => {
            println!("Found Nanoleaf Aurora device at {}", addr);
        }
        Err(e) => {
            eprintln!("Encountered error while searching for Aurora: {}", e);
        }
    }
}
