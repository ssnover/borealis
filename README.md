# borealis

Borealis is a thin interface for interacting with the Nanoleaf Aurora in order
to control your light panels programmatically from Rust. A binary `borealis-query` 
is provided which queries for the IP address of the panels on your local network,
communicates with the Aurora gateway to create a new authorization token for using
the API, and stores the resulting data in a configuration file for reading the
configuration out for using the library client later.

```rust
use borealis::Aurora;
use std::env;
use std::net::Ipv4Addr;

#[tokio::main]
pub async fn main() {
    let effect_name: String = env::args()
        .skip(1)
        .next()
        .expect("Must specify effect name to display.");

    let aurora = Aurora::new(
        Ipv4Addr::new(192, 168, 1, 12),   // IP Address
        None,                             // Port
        &"YourAuthTokenHere".to_string(), // Auth Token
    );

    tokio::spawn(async move {
        println!("Setting Aurora to display {}", effect_name);
        aurora.turn_on().await.unwrap();
        aurora.set_effect(&effect_name).await.unwrap();
    })
    .await
    .unwrap();
}
```