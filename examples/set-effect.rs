use borealis::client::Aurora;
use std::env;
use std::net::Ipv4Addr;

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = env::args().collect();
    let effect_name = args[1].clone();

    let aurora = Aurora::new(
        Ipv4Addr::new(192, 168, 1, 12),
        None,
        &"I8NTBbt5IsFhZ5yAuSaa38m9j70m4odx".to_string(),
    );

    tokio::spawn(async move {
        println!("Setting Aurora to display {}", effect_name);
        aurora.set_effect(&effect_name).await.unwrap();
    })
    .await
    .unwrap();
}
